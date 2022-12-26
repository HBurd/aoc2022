use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Add;
use std::ops::Sub;
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct ResourceGroup
{
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl AddAssign for ResourceGroup
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for ResourceGroup
{
    fn sub_assign(&mut self, rhs: Self)
    {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Add for ResourceGroup
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        let mut result: Self = self;
        result += rhs;
        return result;
    }
}

impl Sub for ResourceGroup
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self
    {
        let mut result: Self = self;
        result -= rhs;
        return result;
    }
}

impl PartialOrd for ResourceGroup
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        if self == other
        {
            Some(Ordering::Equal)
        }
        else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
        {
            Some(Ordering::Greater)
        }
        else if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geode <= other.geode
        {
            Some(Ordering::Less)
        }
        else
        {
            None
        }
    }
}

#[derive(Debug)]
struct Blueprint
{
    ore_robot: ResourceGroup,
    clay_robot: ResourceGroup,
    obsidian_robot: ResourceGroup,
    geode_robot: ResourceGroup,
}

impl Blueprint {
    // Not an implementation of the FromStr trait
    // because it doesn't return a result
    fn from_str(s: &str) -> Blueprint {

        let mut ore_robot_cost: ResourceGroup = Default::default();
        let mut clay_robot_cost: ResourceGroup = Default::default();
        let mut obsidian_robot_cost: ResourceGroup = Default::default();
        let mut geode_robot_cost: ResourceGroup = Default::default();

        for (i, word) in s.split(' ').enumerate() {
            match i {
                6  => { ore_robot_cost.ore = word.parse().unwrap() },
                12 => { clay_robot_cost.ore = word.parse().unwrap() },
                18 => { obsidian_robot_cost.ore = word.parse().unwrap() },
                21 => { obsidian_robot_cost.clay = word.parse().unwrap() },
                27 => { geode_robot_cost.ore = word.parse().unwrap() },
                30 => { geode_robot_cost.obsidian = word.parse().unwrap() },
                _ => {},
            }
        }
        
        return Blueprint {
            ore_robot: ore_robot_cost,
            clay_robot: clay_robot_cost,
            obsidian_robot: obsidian_robot_cost,
            geode_robot: geode_robot_cost,
        };
    }
}

fn maximize_geodes(t: u32, starting_resources: ResourceGroup, robots: ResourceGroup, bp: &Blueprint, skip_ore: bool, skip_clay: bool, skip_obsidian: bool, skip_geode: bool) -> ResourceGroup
{
    let next_resources = starting_resources + robots;

    if t == 1
    {
        return next_resources;
    }
    else if t > 1
    {
        // Default is not building anything
        let mut max_geodes: ResourceGroup = Default::default();

        let mut skip_ore_next = false;
        let mut skip_clay_next = false;
        let mut skip_obsidian_next = false;
        let mut skip_geode_next = false;

        if !skip_ore && starting_resources >= bp.ore_robot
        {
            skip_ore_next = true;
            let max_geodes_ore = maximize_geodes(
                t - 1,
                ResourceGroup { ore: next_resources.ore - bp.ore_robot.ore, ..next_resources },
                ResourceGroup { ore: robots.ore + 1, clay: robots.clay, obsidian: robots.obsidian, geode: robots.geode },
                bp, false, false, false, false
            );
            if max_geodes_ore.geode > max_geodes.geode
            {
                max_geodes = max_geodes_ore;
            }
        }
        if !skip_clay && starting_resources >= bp.clay_robot
        {
            skip_clay_next = true;
            let max_geodes_clay = maximize_geodes(
                t - 1,
                ResourceGroup { ore: next_resources.ore - bp.clay_robot.ore, ..next_resources },
                ResourceGroup { ore: robots.ore, clay: robots.clay + 1, obsidian: robots.obsidian, geode: robots.geode },
                bp, false, false, false, false
            );
            if max_geodes_clay.geode > max_geodes.geode
            {
                max_geodes = max_geodes_clay;
            }
        }
        if !skip_obsidian && starting_resources >= bp.obsidian_robot
        {
            skip_obsidian_next = true;
            let max_geodes_obsidian = maximize_geodes(
                t - 1,
                ResourceGroup { ore: next_resources.ore - bp.obsidian_robot.ore, clay: next_resources.clay - bp.obsidian_robot.clay, ..next_resources },
                ResourceGroup { ore: robots.ore, clay: robots.clay, obsidian: robots.obsidian + 1, geode: robots.geode },
                bp, false, false, false, false
            );
            if max_geodes_obsidian.geode > max_geodes.geode
            {
                max_geodes = max_geodes_obsidian;
            }
        }
        if !skip_geode && starting_resources >= bp.geode_robot
        {
            skip_geode_next = true;
            let max_geodes_geode = maximize_geodes(
                t - 1,
                ResourceGroup { ore: next_resources.ore - bp.geode_robot.ore, obsidian: next_resources.obsidian - bp.geode_robot.obsidian, ..next_resources },
                ResourceGroup { ore: robots.ore, clay: robots.clay, obsidian: robots.obsidian, geode: robots.geode + 1 },
                bp, false, false, false, false
            );
            if max_geodes_geode.geode > max_geodes.geode
            {
                max_geodes = max_geodes_geode;
            }
        }

        if !skip_ore_next || !skip_clay_next || !skip_obsidian_next || !skip_geode_next
        {
            let max_geodes_none = maximize_geodes(t - 1, next_resources, robots, bp, skip_ore_next, skip_clay_next, skip_obsidian_next, skip_geode_next);
            if max_geodes_none.geode > max_geodes.geode
            {
                max_geodes = max_geodes_none;
            }
        }

        return max_geodes;
    }
    else {
        panic!("t should be greater than zero!");
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::from_str).collect();

    let mut quality = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let resources: ResourceGroup = Default::default();
        let robots = ResourceGroup {
            ore: 1,
            ..Default::default()
        };

        let geodes = maximize_geodes(32, resources, robots, blueprint, false, false, false, false);
        println!("Resources: {:?}", geodes);

        quality += (i + 1) * geodes.geode as usize;
    }
    
    println!("Quality: {}", quality);
}
