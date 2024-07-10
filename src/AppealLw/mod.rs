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

unsafe extern "C" fn reflet_appeallwl(agent: &mut L2CAgentBase) {
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
            Hash40::new("appeal_lw_l"),
            false,
            -1.0,
        );
    }
}

unsafe extern "C" fn reflet_appeallwr(agent: &mut L2CAgentBase) {
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
            Hash40::new("appeal_lw_l"),
            false,
            -1.0,
        );
    }
}

unsafe extern "C" fn chrom_appeallwl(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        refletPosX[entry_id] = PostureModule::pos_x(own_boma) - 10.0;
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

unsafe extern "C" fn chrom_appeallwr(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        refletPosX[entry_id] = PostureModule::pos_x(agent.module_accessor) + 10.0;
        refletPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        refletPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);

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

unsafe extern "C" fn chrom_effect_appeallw(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 8);
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            -7,
            0,
            -10,
            0,
            -8,
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
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            5,
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
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            5,
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
    frame(agent.lua_state_agent, 99.0);
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
        .game_acmd("game_appeallwl", reflet_appeallwl, Priority::Low)
        .game_acmd("game_appeallwr", reflet_appeallwr, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_appeallwl", chrom_appeallwl, Priority::Low)
        .game_acmd("game_appeallwr", chrom_appeallwr, Priority::Low)
        .effect_acmd("effect_appeallwl", chrom_effect_appeallw, Priority::Default)
        .install();
}
