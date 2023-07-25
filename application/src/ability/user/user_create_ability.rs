use base::model::ddd::marker_interface::MarkerInterface;
use domain::{
    aggregate::{
        role::{model::role::Role, repository::role_repository::IRoleRepository},
        user::{
            iservice::iuser_domain_service::IUserDomainService,
            repository::user_repository::IUserRepository,
        },
    },
    share::event::{
        base_domain_event::IBaseDomainEvent, idomain_event_publisher::IDomainEventPublisher,
    },
};

use crate::ability::share::base_ability::BaseAbility;

use super::cmd::create_user_ability_command::CreateUserAbilityCommand;

pub struct UserCreateCtx {
    pub roles: Vec<Role>,
}

pub struct UserCreateAbility<RR, UR, UDS, BDE, DEP>
where
    RR: IRoleRepository,
    UR: IUserRepository,
    UDS: IUserDomainService,
    BDE: IBaseDomainEvent,
    DEP: IDomainEventPublisher<BDE>,
{
    roleRepository: RR,
    userRepository: UR,
    userDomainService: UDS,
    userCreateEvent: BDE,
    domainEventPublisher: DEP,
    ctx: UserCreateCtx,
}

impl<RR, UR, UDS, BDE, DEP> MarkerInterface for UserCreateAbility<RR, UR, UDS, BDE, DEP>
where
    RR: IRoleRepository,
    UR: IUserRepository,
    UDS: IUserDomainService,
    BDE: IBaseDomainEvent,
    DEP: IDomainEventPublisher<BDE>,
{
}

#[async_trait::async_trait]
impl<RR, UR, UDS, BDE, DEP> BaseAbility<CreateUserAbilityCommand, ()>
    for UserCreateAbility<RR, UR, UDS, BDE, DEP>
where
    RR: IRoleRepository,
    UR: IUserRepository,
    UDS: IUserDomainService,
    BDE: IBaseDomainEvent,
    DEP: IDomainEventPublisher<BDE>,
{
    async fn check_handler(&mut self, cmd: CreateUserAbilityCommand) -> anyhow::Result<()> {
        // match self.userRepository.by_user_name(cmd.user_name).await {
        //     Ok(_r) => {}
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };
        // let roles = self.roleRepository.list_by_ids(cmd.roles).await;
        // self.ctx.roles = roles;
        return Ok(());
    }

    async fn check_idempotent(&self, _cmd: CreateUserAbilityCommand) -> anyhow::Result<()> {
        Ok(())
    }

    async fn execute(&self, cmd: CreateUserAbilityCommand) -> anyhow::Result<()> {
        // let user = cmd.to_user();
        // // 执行用户新增相关业务逻辑
        // user.print_create();
        // let user = match __self.userRepository.save(user).await {
        //     Ok(r) => r,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        // //仅仅为了演示领域服务使用，这没必要这么做，能力点已经是一个比较原子的业务逻辑点了
        // //理论上有了能力层之后直接可以砍掉领域服务层
        // self.userDomainService
        //     .print_tag(user.clone(), self.ctx.roles.clone());

        // let save = match __self.userRepository.save(user).await {
        //     Ok(r) => r,
        //     Err(e) => return Err(e),
        // };

        todo!();
    }

    async fn execute_ability(&self, cmd: CreateUserAbilityCommand) -> anyhow::Result<()> {
        todo!()
    }
}

// pub struct ABC {
//     pub a: String,
// }

// #[async_trait::async_trait]
// impl BaseAbility<CreateUserAbilityCommand, ()> for ABC {
//     async fn execute_ability(&self, cmd: CreateUserAbilityCommand) -> Result<()> {
//         Ok(())
//     }
//     async fn check_handler(&mut self, cmd: CreateUserAbilityCommand) -> Result<()> {
//         Ok(())
//     }
//     async fn check_idempotent(&self, cmd: CreateUserAbilityCommand) -> Result<()> {
//         Ok(())
//     }
//     async fn execute(&self, cmd: CreateUserAbilityCommand) -> Result<()> {
//         Ok(())
//     }
// }
