use anyhow::Result;
use base::model::ddd::marker_interface::MarkerInterface;

#[async_trait::async_trait]
pub trait BaseAbility<T, R>: MarkerInterface {
    async fn execute_ability(&self, cmd: T) -> Result<R>;
    /// 能力点执行前的参数校验
    async fn check_handler(&mut self, cmd: T) -> Result<()>;
    /// 能力点执行前的幂等校验 false：当前能力点已执行，不再执行业务逻辑、true：当前能力点未执行，继续执行业务逻辑
    /// 数据幂等的校验，例如订单已经支付则不在进行支付。通过返回结果的方式中断流程，由应用服务决定如何处理。
    async fn check_idempotent(&self, cmd: T) -> Result<R>;
    async fn execute(&self, cmd: T) -> Result<R>;
}

// pub struct AbilityContext {
//     pub key1: String,
// }

// impl AbilityContext {
//     pub fn init() -> Self {
//         return AbilityContext {
//             key1: String::from(""),
//         };
//     }
// }
