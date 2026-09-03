use bevy::prelude::*;

/// 軸座標系 (Axial Coordinates: q, r)
/// Pointy-topped (上が尖った六角形) を採用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub const fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// 立方体座標の s 成分 (q + r + s = 0)
    #[inline]
    pub const fn s(&self) -> i32 {
        -self.q - self.r
    }

    /// 2つのヘックス間の距離（タイル数）
    #[allow(dead_code)]
    pub fn distance(&self, other: HexCoord) -> i32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = (self.s() - other.s()).abs();
        (dq + dr + ds) / 2
    }

    /// 隣接する6つのヘックス座標を取得
    #[allow(dead_code)]
    pub fn neighbors(&self) -> [HexCoord; 6] {
        const DIRECTIONS: [(i32, i32); 6] = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
        ];

        let mut res = [*self; 6];
        for i in 0..6 {
            let (dq, dr) = DIRECTIONS[i];
            res[i] = HexCoord::new(self.q + dq, self.r + dr);
        }
        res
    }

    /// ヘックス座標から 3D ワールド座標 (X, 0.0, Z) を計算
    /// pointy-topped hex の場合:
    /// x = size * sqrt(3) * (q + r / 2)
    /// z = size * 3/2 * r
    pub fn to_world_pos(self, hex_radius: f32) -> Vec3 {
        let sqrt3 = 3.0f32.sqrt();
        let x = hex_radius * sqrt3 * (self.q as f32 + self.r as f32 / 2.0);
        let z = hex_radius * 1.5 * (self.r as f32);
        Vec3::new(x, 0.0, z)
    }

    /// 3D ワールド座標 (X, Z) から最も近い HexCoord を逆算（Pointy-topped）
    pub fn from_world_pos(pos: Vec3, hex_radius: f32) -> Self {
        let sqrt3 = 3.0f32.sqrt();
        let q_frac = (sqrt3 / 3.0 * pos.x - 1.0 / 3.0 * pos.z) / hex_radius;
        let r_frac = (2.0 / 3.0 * pos.z) / hex_radius;
        let s_frac = -q_frac - r_frac;

        let mut q = q_frac.round() as i32;
        let mut r = r_frac.round() as i32;
        let s = s_frac.round() as i32;

        let q_diff = (q as f32 - q_frac).abs();
        let r_diff = (r as f32 - r_frac).abs();
        let s_diff = (s as f32 - s_frac).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        }

        Self::new(q, r)
    }
}
