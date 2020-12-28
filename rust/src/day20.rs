use crate::common;
use std::collections::HashMap;
use std::fmt;
use std::fs;

type TileId = u64;

#[derive(Debug)]
struct Tile {
    pt: Vec<Vec<char>>,
    id: TileId,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
    fn opposite_idx(&self) -> usize {
        Dir::ENUM
            .iter()
            .position(|d| *d == self.opposite())
            .unwrap()
    }
    const ENUM: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];
    const OFFS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
}

#[derive(Debug, Clone)]
struct Transform {
    rotation: Dir,
    flip_x: bool,
    flip_y: bool,
}

type TileSet = Vec<Tile>;

#[derive(Debug, Clone)]
struct TileInstance {
    tile_idx: usize,
    tm: Transform,
    edges: [String; 4],
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct TileMappingKey {
    dir: Dir,
    edge: String,
}

type TileMapping = HashMap<TileMappingKey, Vec<TileInstance>>;
type TileLayout = HashMap<(i64, i64), TileInstance>;

impl Tile {
    fn parse(text: &str, has_header: bool) -> Option<Tile> {
        let lines: Vec<&str> = text.split("\n").map(|s| s.trim()).collect();
        if lines.len() < 2 {
            return None;
        }
        let (id, rows) = if has_header {
            let header = lines[0].split(" ").collect::<Vec<&str>>()[1];
            (
                header[..header.len() - 1].parse().ok()?,
                lines[1..].to_vec(),
            )
        } else {
            (0, lines[0..].to_vec())
        };
        Some(Tile {
            id: id,
            pt: rows.iter().map(|s| s.chars().collect()).collect(),
        })
    }
    fn empty(w: usize, h: usize) -> Tile {
        Tile {
            id: 0,
            pt: vec![vec!['.'; w]; h],
        }
    }
    fn count(&self, ch: char) -> usize {
        self.pt
            .iter()
            .flat_map(|row| row.iter().map(|c| (*c == ch) as usize))
            .sum()
    }
    fn get_extents(&self) -> (usize, usize) {
        (self.pt[0].len(), self.pt.len())
    }
    fn get_edge(&self, dir: &Dir) -> String {
        match dir {
            Dir::N => self.pt[0].iter().collect(),
            Dir::E => self.pt.iter().map(|s| *s.last().unwrap()).collect(),
            Dir::S => self.pt.last().unwrap().iter().collect(),
            Dir::W => self.pt.iter().map(|s| s[0]).collect(),
        }
    }
    fn get_edges(&self) -> [String; 4] {
        [
            self.get_edge(&Dir::N),
            self.get_edge(&Dir::E),
            self.get_edge(&Dir::S),
            self.get_edge(&Dir::W),
        ]
    }
    fn transform(&self, tm: &Transform) -> Tile {
        let n = self.pt.len();
        let mut res = Tile::empty(n, n);
        if n == 0 {
            return res;
        }
        assert!(n == self.pt[0].len());
        for j in 0..n {
            for i in 0..n {
                let (mut x, mut y) = (i, j);
                if tm.flip_x {
                    x = n - x - 1;
                }
                if tm.flip_y {
                    y = n - y - 1;
                }
                let (rx, ry) = match tm.rotation {
                    Dir::N => (x, y),
                    Dir::E => (y, n - x - 1),
                    Dir::S => (n - x - 1, n - y - 1),
                    Dir::W => (n - y - 1, x),
                };
                res.pt[j][i] = self.pt[ry][rx];
            }
        }
        res
    }
    fn blit(&mut self, tile: &Tile, x: usize, y: usize, border: usize) {
        let n = tile.pt.len();
        for j in 0..(n - border * 2) {
            for i in 0..(n - border * 2) {
                self.pt[y + j][x + i] = tile.pt[j + border][i + border];
            }
        }
    }
    fn matches(&self, tile: &Tile, x: usize, y: usize) -> bool {
        for (j, row) in tile.pt.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if *c == '#' && self.pt[y + j][x + i] != *c {
                    return false;
                }
            }
        }
        true
    }
    fn find_matches(&self, pattern: &Tile) -> Vec<(i64, i64)> {
        let mut res = Vec::new();
        let (w, h) = self.get_extents();
        let (tw, th) = pattern.get_extents();
        for j in 0..(h - th + 1) {
            for i in 0..(w - tw + 1) {
                if self.matches(pattern, i, j) {
                    res.push((i as i64, j as i64));
                }
            }
        }
        res
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.pt {
            for c in row.iter() {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn create_mapping(tiles: &TileSet) -> TileMapping {
    let mut res = TileMapping::new();
    for (i, tile) in tiles.iter().enumerate() {
        for flip_x in &[false, true] {
            for flip_y in &[false, true] {
                if *flip_x && *flip_y {
                    continue;
                }
                for rotation in &Dir::ENUM {
                    let tm = Transform {
                        rotation: rotation.clone(),
                        flip_x: *flip_x,
                        flip_y: *flip_y,
                    };
                    let tile_transformed = tile.transform(&tm);
                    for dir in &Dir::ENUM {
                        let edge = tile_transformed.get_edge(dir);
                        let key = TileMappingKey {
                            dir: dir.clone(),
                            edge: edge,
                        };
                        let tile_instance = TileInstance {
                            tile_idx: i,
                            tm: tm.clone(),
                            edges: tile_transformed.get_edges(),
                        };
                        res.entry(key).or_insert(Vec::new()).push(tile_instance);
                    }
                }
            }
        }
    }
    res
}

fn find_corners(tiles: &TileSet, mapping: &TileMapping) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        let mut num_unique_edges = 0;
        for dir in &Dir::ENUM {
            let key = TileMappingKey {
                dir: dir.opposite(),
                edge: tile.get_edge(dir),
            };
            num_unique_edges += (match mapping.get(&key) {
                Some(matches) => matches.iter().map(|t| (i != t.tile_idx) as u32).sum(),
                _ => 0,
            } == 0) as u32;
        }
        if num_unique_edges == 2 {
            res.push(i);
        }
    }
    res
}

fn get_layout(tiles: &TileSet, mapping: &TileMapping) -> TileLayout {
    fn rec(
        tile_idx: usize,
        tm: &Transform,
        pos: (i64, i64),
        tiles: &TileSet,
        mapping: &TileMapping,
        used: &mut Vec<bool>,
        res: &mut TileLayout,
    ) -> bool {
        if used[tile_idx] {
            return false;
        }
        // find if existing neighbors fit with the transformed tile's edges
        let tile_transformed = tiles[tile_idx].transform(&tm);
        let edges = tile_transformed.get_edges();
        for (i, edge) in edges.iter().enumerate() {
            let offs = Dir::OFFS[i];
            let pos1 = (pos.0 + offs.0, pos.1 + offs.1);
            match res.get(&pos1) {
                Some(t) => {
                    if t.edges[Dir::ENUM[i].opposite_idx()] != *edge {
                        return false;
                    }
                }
                _ => (),
            }
        }
        used[tile_idx] = true;
        res.insert(
            pos,
            TileInstance {
                tile_idx: tile_idx,
                tm: tm.clone(),
                edges: edges.clone(),
            },
        );
        if res.len() == tiles.len() {
            return true;
        }
        // recurse into neighbors
        let mut found_fit = false;
        for (i, edge) in edges.iter().enumerate() {
            let offs = Dir::OFFS[i];
            let pos1 = (pos.0 + offs.0, pos.1 + offs.1);
            if res.contains_key(&pos1) {
                // neighbor position is already occupied
                continue;
            }
            let key = TileMappingKey {
                dir: Dir::ENUM[i].opposite(),
                edge: edge.clone(),
            };
            match mapping.get(&key) {
                Some(instances) => {
                    if instances
                        .iter()
                        .any(|inst| rec(inst.tile_idx, &inst.tm, pos1, tiles, mapping, used, res))
                    {
                        found_fit = true;
                        break;
                    }
                }
                _ => (),
            }
        }
        if !found_fit {
            used[tile_idx] = false;
            res.remove(&pos);
        }
        found_fit
    }
    let mut res = TileLayout::new();
    let mut used = vec![false; tiles.len()];
    let corners = find_corners(tiles, mapping);
    let start_idx = corners[0];
    rec(
        start_idx,
        &Transform {
            rotation: Dir::N,
            flip_x: false,
            flip_y: false,
        },
        (0, 0),
        tiles,
        mapping,
        &mut used,
        &mut res,
    );
    res
}

fn blit_layout(tiles: &TileSet, layout: &TileLayout) -> Tile {
    let minx = layout.keys().map(|x| x.0).min().unwrap();
    let maxx = layout.keys().map(|x| x.0).max().unwrap();
    let miny = layout.keys().map(|x| x.1).min().unwrap();
    let maxy = layout.keys().map(|x| x.1).max().unwrap();
    let n = tiles[0].pt.len() - 2;
    let mut res = Tile::empty(
        n * (maxx - minx + 1) as usize,
        n * (maxy - miny + 1) as usize,
    );
    for (pos, instance) in layout {
        let tile_transformed = tiles[instance.tile_idx].transform(&instance.tm);
        res.blit(
            &tile_transformed,
            n * (pos.0 - minx) as usize,
            n * (pos.1 - miny) as usize,
            1,
        );
    }
    res
}

fn find_pattern(img: &Tile, pattern: &Tile) -> Vec<(i64, i64)> {
    for flip_x in &[false, true] {
        for flip_y in &[false, true] {
            if *flip_x && *flip_y {
                continue;
            }
            for rotation in &Dir::ENUM {
                let tm = Transform {
                    rotation: rotation.clone(),
                    flip_x: *flip_x,
                    flip_y: *flip_y,
                };
                let img1 = img.transform(&tm);
                let matches = img1.find_matches(pattern);
                if matches.len() > 0 {
                    return matches;
                }
            }
        }
    }
    Vec::new()
}

pub(crate) fn solution() {
    let tiles: TileSet = fs::read_to_string(&common::data_file(20))
        .unwrap()
        .split("\n\n")
        .filter_map(|s| Tile::parse(s, true))
        .collect();
    let mapping = create_mapping(&tiles);
    let corners = find_corners(&tiles, &mapping);
    let res1: u64 = corners.iter().map(|i| tiles[*i].id).product();
    println!("Answer 1: {}", res1);

    let pattern = Tile::parse(
        &fs::read_to_string(&common::extra_file(20, "pattern")).unwrap(),
        false,
    )
    .unwrap();
    let layout = get_layout(&tiles, &mapping);
    let img = blit_layout(&tiles, &layout);
    let m = find_pattern(&img, &pattern);
    println!(
        "Answer 2: {}",
        img.count('#') - m.len() * pattern.count('#')
    );
}
