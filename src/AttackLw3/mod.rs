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

unsafe extern "C" fn reflet_attacklw3(agent: &mut L2CAgentBase) {
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
            Hash40::new("attack_lw3"),
            false,
            -1.0,
        );
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("sword"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

unsafe extern "C" fn chrom_attacklw3(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(
        agent.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID,
    ) as usize;
    let own_boma = sv_battle_object::module_accessor(entry_id as u32);
    if macros::is_excute(agent) {
        if PostureModule::lr(own_boma) < 0.0 {
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
        } else {
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
}

unsafe extern "C" fn chrom_effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
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
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_chrom_sword1"),
            Hash40::new("tex_chrom_sword2"),
            9,
            Hash40::new("sword1"),
            0,
            0,
            1.7,
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            12.6,
            true,
            Hash40::new("chrom_sword"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.2,
            0.2,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }

    frame(agent.lua_state_agent, 51.0);
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
        .expression_acmd("expression_attacklw3", reflet_attacklw3, Priority::Low)
        .install();
    Agent::new("reflet_chrom")
        .game_acmd("game_attacklw3", chrom_attacklw3, Priority::Low)
        .effect_acmd("effect_attacklw3", chrom_effect_attacklw3, Priority::Low)
        .install();
}
