use bevy::prelude::*;

/// 横ループマップのサイズ定義
pub const MAP_WIDTH: i32 = 28;  // 横方向タイル数（東西方向、ループする）
pub const MAP_HEIGHT: i32 = 16; // 縦方向タイル数（南北方向、-MAP_HEIGHT/2 .. MAP_HEIGHT/2）

/// 軸座標系 (Axial Coordinates: q, r)
/// Pointy-topped (上が尖った六角形) を採用
/// 横方向のオフセット座標 (col, row):
/// row = r
/// col = q + (r - (r & 1)) / 2
/// 逆変換:
/// q = col - (row - (row & 1)) / 2
/// r = row
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

    /// 横ループの正規化を行わない（ラップ前のcolを指定した）HexCoordを作成
    pub fn from_col_row_unwrapped(col: i32, row: i32) -> Self {
        let q = col - (row - (row & 1)) / 2;
        Self { q, r: row }
    }

    /// (col, row) のオフセット座標からHexCoordを作成
    pub fn from_col_row(col: i32, row: i32) -> Self {
        Self::from_col_row_unwrapped(col.rem_euclid(MAP_WIDTH), row)
    }

    /// 横ループの正規化を行わない (col, row) のオフセット座標を取得
    pub fn to_col_row_unwrapped(self) -> (i32, i32) {
        let row = self.r;
        let col = self.q + (row - (row & 1)) / 2;
        (col, row)
    }

    /// (col, row) のオフセット座標を取得 (col は 0..MAP_WIDTH-1 に正規化)
    pub fn to_col_row(self) -> (i32, i32) {
        let (col, row) = self.to_col_row_unwrapped();
        (col.rem_euclid(MAP_WIDTH), row)
    }

    /// 横ループを考慮して正規化された HexCoord を取得
    pub fn wrapped(self) -> Self {
        let (col, row) = self.to_col_row();
        Self::from_col_row(col, row)
    }

    /// 立方体座標の s 成分 (q + r + s = 0)
    #[inline]
    pub const fn s(self) -> i32 {
        -self.q - self.r
    }

    /// 2つのヘックス間の距離（タイル数、横ループ考慮）
    #[allow(dead_code)]
    pub fn distance(self, other: HexCoord) -> i32 {
        let (_c1, _r1) = self.to_col_row();
        let (c2, r2) = other.to_col_row();

        // 3つの可能性（そのまま、東へ1周、西へ1周）のうち最小の距離を取る
        let offsets = [-MAP_WIDTH, 0, MAP_WIDTH];
        let mut min_dist = i32::MAX;

        for offset in offsets {
            let shifted_other = HexCoord::from_col_row_unwrapped(c2 + offset, r2);
            let dq = (self.q - shifted_other.q).abs();
            let dr = (self.r - shifted_other.r).abs();
            let ds = (self.s() - shifted_other.s()).abs();
            let d = (dq + dr + ds) / 2;
            if d < min_dist {
                min_dist = d;
            }
        }
        min_dist
    }

    /// 隣接する6つのヘックス座標を取得（横ループ考慮でラップ）
    pub fn neighbors(self) -> [HexCoord; 6] {
        const DIRECTIONS: [(i32, i32); 6] = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
        ];

        let mut res = [self; 6];
        for i in 0..6 {
            let (dq, dr) = DIRECTIONS[i];
            let raw_coord = HexCoord::new(self.q + dq, self.r + dr);
            res[i] = raw_coord.wrapped();
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

    /// 横ループの周回オフセット（-1, 0, 1 など）を加味したワールド座標を計算
    #[allow(dead_code)]
    pub fn to_world_pos_with_wrap(self, hex_radius: f32, wrap_offset: i32) -> Vec3 {
        let mut pos = self.to_world_pos(hex_radius);
        pos.x += wrap_offset as f32 * map_world_width(hex_radius);
        pos
    }

    /// 3D ワールド座標 (X, Z) から最も近い HexCoord を逆算（Pointy-topped）
    /// 横ループを考慮して正規化
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

        HexCoord::new(q, r).wrapped()
    }
}

/// マップのワールド空間における1周分の横幅（X軸メートル）
pub fn map_world_width(hex_radius: f32) -> f32 {
    let sqrt3 = 3.0f32.sqrt();
    hex_radius * sqrt3 * (MAP_WIDTH as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_around() {
        let coord1 = HexCoord::from_col_row(0, 0);
        let coord2 = HexCoord::from_col_row(MAP_WIDTH, 0);
        assert_eq!(coord1, coord2);

        let coord3 = HexCoord::from_col_row(-1, 0);
        let coord4 = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        assert_eq!(coord3, coord4);
    }

    #[test]
    fn test_wrap_distance() {
        let left_edge = HexCoord::from_col_row(0, 0);
        let right_edge = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        // 東西の端同士の距離は横ループにより 1 になる
        assert_eq!(left_edge.distance(right_edge), 1);
    }

    #[test]
    fn test_neighbors_wrap() {
        let left_edge = HexCoord::from_col_row(0, 0);
        let neighbors = left_edge.neighbors();
        let right_edge = HexCoord::from_col_row(MAP_WIDTH - 1, 0);
        assert!(neighbors.contains(&right_edge));
    }
}
