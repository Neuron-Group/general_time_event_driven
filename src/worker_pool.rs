use crate::{event_queue, types::*, widget_queue::*};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

const BFFR_LEN: usize = 10000;

struct WkrHndl<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> {
    wdgt_sndr:
        mpsc::Sender<Box<dyn WdgtT<TmStmpTp = TmStmpTp, EvntTp = EvntTp, WkrPpty = WkrPpty>>>,
    prcs_hndl: JoinHandle<()>,
    // evnt_slctr: Box<dyn FnOnce(&EvntTp) -> bool + Send + Sync>,
}

impl<TmStmpTp: Ord + 'static, EvntTp: EvntTpT + 'static, WkrPpty: WkrPptyT + 'static>
    WkrHndl<TmStmpTp, EvntTp, WkrPpty>
{
    fn new(
        mut evnt_rcvr: broadcast::Receiver<Arc<BoxdEvnt<TmStmpTp, EvntTp>>>,
        // wdgt_rcvr: mpsc::Receiver<BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>>,
        evnt_wkr_mod: WkrMod,
        rt_evnt_sndr: mpsc::Sender<RtEvnt>,
        rt_wdgt_sndr_pre: mpsc::Sender<BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>>,
        evnt_slctr: Box<dyn Fn(&EvntTp) -> bool + Send + Sync>,
    ) -> Self {
        let (wdgt_sndr, mut wdgt_rcvr) = mpsc::channel(BFFR_LEN);
        Self {
            wdgt_sndr,
            prcs_hndl: tokio::spawn(async move {
                let mut wdgt_heap = WdgtHeap::new();
                while let Ok(evnt) = evnt_rcvr.recv().await {
                    while !wdgt_rcvr.is_empty() {
                        wdgt_heap.push(wdgt_rcvr.recv().await.unwrap());
                    }
                    if evnt_slctr(&evnt.as_ref().get_evnt_ppt()) {
                        match evnt_wkr_mod {
                            WkrMod::PrcsOnce => {
                                if !wdgt_heap.is_empty()
                                    && let Some(wdgt) = wdgt_heap.peek()
                                    && wdgt.time_stamp() <= evnt.time_stamp()
                                {
                                    let mut wdgt = wdgt_heap.pop().unwrap();
                                    let rt_stt = wdgt.judge(evnt.as_ref());
                                    match rt_stt {
                                        RtStt::Pending(rt_evnt) => {
                                            let _ = rt_evnt_sndr.send(rt_evnt).await;
                                            let _ = rt_wdgt_sndr_pre.send(wdgt).await;
                                        }
                                        RtStt::Ready(rt_evnt) => {
                                            let _ = rt_evnt_sndr.send(rt_evnt).await;
                                        }
                                    }
                                }
                            }
                            WkrMod::PrcsMltiTimes => {
                                while !wdgt_heap.is_empty() {
                                    if let Some(wdgt) = wdgt_heap.peek()
                                        && wdgt.time_stamp() <= evnt.time_stamp()
                                    {
                                        let mut wdgt = wdgt_heap.pop().unwrap();
                                        let rt_stt = wdgt.judge(evnt.as_ref());
                                        match rt_stt {
                                            RtStt::Pending(rt_evnt) => {
                                                let _ = rt_evnt_sndr.send(rt_evnt).await;
                                                let _ = rt_wdgt_sndr_pre.send(wdgt).await;
                                            }
                                            RtStt::Ready(rt_evnt) => {
                                                let _ = rt_evnt_sndr.send(rt_evnt).await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }),
        }
    }
}

/// 工作池结构体，管理事件处理和组件路由
///
/// 负责协调事件分发、组件处理和结果返回，内部包含多个工作线程和路由机制
///
/// # 泛型参数
/// * `TmStmpTp` - 浮点类型，需实现Ord
/// * `EvntTp` - 事件类型，需实现EvntTpT
/// * `WkrPpty` - 工作属性类型，需实现WkrPptyT
pub struct WkrPool<TmStmpTp: Ord, EvntTp: EvntTpT, WkrPpty: WkrPptyT> {
    // 优先队列线程
    ipt_wkr_hndl: JoinHandle<()>,
    _evnt_brdcst_sndr: broadcast::Sender<Arc<BoxdEvnt<TmStmpTp, EvntTp>>>,

    // 哈希表路由线程
    wdgt_rotr_hndl: JoinHandle<()>,

    // 预留
    _rt_wdgt_sndr_pre: mpsc::Sender<BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>>,
}

impl<TmStmpTp: Ord + 'static, EvntTp: EvntTpT + 'static, WkrPpty: WkrPptyT + 'static>
    WkrPool<TmStmpTp, EvntTp, WkrPpty>
{
    /// 构建工作池实例
    ///
    /// 创建事件通道、广播器和工作线程，返回发送器、接收器和工作池实例
    ///
    /// # 参数
    /// * `wkr_ppty` - 工作属性列表，包含工作属性、工作模式和事件选择器
    ///
    /// # 返回值
    /// 元组：(事件发送器, 运行时事件接收器, WkrPool实例)
    pub fn build(
        wkr_ppty: Vec<(WkrPpty, WkrMod, Box<dyn Fn(&EvntTp) -> bool + Send + Sync>)>,
        wdgts: Vec<BoxdWdgt<TmStmpTp, EvntTp, WkrPpty>>,
    ) -> (
        event_queue::Sndr<TmStmpTp, EvntTp>,
        mpsc::Receiver<RtEvnt>,
        Self,
    ) {
        let (evnt_pipe_sndr, evnt_pipe_rcvr) = event_queue::chnl();
        let (evnt_tx, _) = broadcast::channel(BFFR_LEN);
        let (rt_evnt_sndr, rt_evnt_rcvr) = mpsc::channel(BFFR_LEN);
        let (rt_wdgt_sndr_pre, mut rt_wdgt_rcvr_pre) = mpsc::channel(BFFR_LEN);
        let wkrs_tabl = wkr_ppty
            .into_iter()
            .map(|e| {
                (
                    e.0,
                    WkrHndl::new(
                        evnt_tx.subscribe(),
                        e.1,
                        rt_evnt_sndr.clone(),
                        rt_wdgt_sndr_pre.clone(),
                        e.2,
                    ),
                )
            })
            .collect::<HashMap<WkrPpty, WkrHndl<TmStmpTp, EvntTp, WkrPpty>>>();

        let evnt_brdcst_sndr = evnt_tx.clone();

        let ipt_wkr_hndl = tokio::spawn(async move {
            loop {
                let evnt = evnt_pipe_rcvr.recv().await;
                if evnt_tx.send(Arc::new(evnt)).is_err() {
                    break;
                }
            }
        });

        let wdgt_rotr_hndl = tokio::spawn(async move {
            loop {
                let wdgt = rt_wdgt_rcvr_pre.recv().await;
                if let Some(wdgt) = wdgt
                    && let Some(wkr_hndl) = wkrs_tabl.get(&wdgt.get_wkr_ppt())
                    && wkr_hndl.wdgt_sndr.send(wdgt).await.is_err()
                {
                    break;
                }
            }
        });

        wdgts
            .into_iter()
            .for_each(|e| rt_wdgt_sndr_pre.blocking_send(e).unwrap());

        (
            evnt_pipe_sndr,
            rt_evnt_rcvr,
            Self {
                ipt_wkr_hndl,
                wdgt_rotr_hndl,
                _evnt_brdcst_sndr: evnt_brdcst_sndr,
                _rt_wdgt_sndr_pre: rt_wdgt_sndr_pre,
            },
        )
    }
}
