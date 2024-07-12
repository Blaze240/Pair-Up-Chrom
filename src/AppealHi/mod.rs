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

static mut refletPosX: [f32; 8] = [0.0; 8];
static mut refletPosY: [f32; 8] = [0.0; 8];
static mut refletPosZ: [f32; 8] = [0.0; 8];

unsafe extern "C" fn reflet_appealhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            false,
            -1,
        );
        ArticleModule::change_motion(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            Hash40::new("appeal_hi_l"),
            false,
            -1.0,
        );
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(
            agent.module_accessor,
            *FIGHTER_REFLET_GENERATE_ARTICLE_CHROM,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
        );
    }
}

unsafe extern "C" fn chrom_appealhi(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        refletPosX[entry_id] = PostureModule::pos_x(own_boma) + 10.0;
        refletPosY[entry_id] = PostureModule::pos_y(own_boma);
        refletPosZ[entry_id] = PostureModule::pos_z(own_boma);

        PostureModule::set_pos(
            agent.module_accessor,
            &Vector3f {
                x: refletPosX[entry_id],
                y: refletPosY[entry_id],
                z: refletPosZ[entry_id],
            },
        );
    }
}

unsafe extern "C" fn chrom_effect_appealhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("reflet_entry"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            270,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            10,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            7,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("finreflet_warp"),
            Hash40::new("top"),
            0.0,
            0.0,
            0.0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
}

pub fn install() {
    Agent::new("reflet")
        .game_acmd("game_appealhil", reflet_appealhi, Priority::Low)
        .game_acmd("game_appealhir", reflet_appealhi, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_appealhil", chrom_appealhi, Priority::Low)
        .effect_acmd("effect_appealhil", chrom_effect_appealhi, Priority::Low)
        .install();
}
