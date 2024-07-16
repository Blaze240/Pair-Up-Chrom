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

unsafe extern "C" fn reflet_landingheavy(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

pub fn install() {
    Agent::new("reflet")
        .game_acmd("game_landingheavy_pairup", reflet_landingheavy, Priority::Low)
        .install();
}
