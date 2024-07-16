use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn reflet_damagecheckframe(agent: &mut L2CAgentBase) {
    unsafe {
        if MotionModule::motion_kind(agent.module_accessor) == hash40("damage_hi_1")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_hi_2")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_hi_3")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_n_1")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_n_2")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_n_3")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_n_1")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_lw_1")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_lw_2")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_lw_3")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_air_1")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_air_2")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_air_3")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_fly_hi")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_fly_n")
            || MotionModule::motion_kind(agent.module_accessor) == hash40("damage_fly_lw")
        {
            ArticleModule::remove_exist(
                agent.module_accessor,
                *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
            );
        }
    }
}

pub fn install() {
    Agent::new("reflet")
        .on_line(Main, reflet_damagecheckframe)
        .install();
}
