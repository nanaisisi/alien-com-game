use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {
        // ワールド全体の共通設定や追加環境アセット管理
    }
}

