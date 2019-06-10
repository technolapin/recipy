/*
//#[derive(Debug)]
struct Item
{
    name: String,
    recipy:Vec<(&'static Item, i32)>,
    is_atomic: bool
}


enum Item
{}


impl Item
{

    fn new (name: &str, recipy: Vec<(&'static Item, i32)>, is_atomic: bool) -> Self
    {
        Item
        {
            name: name.to_string(),
            recipy: recipy,
            is_atomic: is_atomic
        }
    }

    fn new_atomic (name: &str) -> Self
    {
        Self::new(name, Vec::new(), true)
    }
    fn new_composed (name: &str, recipy: Vec<(&'static Item, i32)>) -> Self
    {
        Self::new(name, recipy, false)
    }
}

impl std::fmt::Debug for Item
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_atomic
        {
            write!(f, "{}", self.name)
        }
        else
        {
            write!(f, "{}: {:?}", self.name, self.recipy)
        }
    }
}



fn main() {

    let cobble = Item::new_atomic("Cobblestone");

    let furnave = Item::new_composed("Furnace",vec![(&'static cobble, 8)]);

    println!("{:?}", cobble);

    println!("Hello, world!");
}

 */

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Item {
    TinBar,
    IronBar,
    CopperBar,
    LeadBar,
    GoldBar,
    WoodenPlanks,
    Cobblestone,
    Furnace,
    IronFurnace,
    RefinedIron,
    CopperCable,
    InsulatedCopperCable,
    Rubber,
    REBattery,
    Generator,
    Redstone,
    ElectronicCircuit,
    PulverizedCoal,
    Glass,
    SolarPanel,
    LVTransformer,
    MVTransformer,
    BasicMachineCasing,
    LVSolarArray,
    GoldCable,
    InsulatedGoldCable,
    TwiceInsulatedGoldCable,
    MVSolarArray,
}
use Item::*;

impl Item {
    fn to_string(self) -> String {
        String::from(match self {
            TinBar => "Tin Bar",
            IronBar => "Iron bar",
            CopperBar => "Copper bar",
            LeadBar => "Lead bar",
            GoldBar => "Gold bar",
            WoodenPlanks => "Wooden plank",
            Cobblestone => "Cobblestone",
            Furnace => "Furnace",
            IronFurnace => "Iron furnace",
            RefinedIron => "Refined Iron",
            CopperCable => "Copper cable",
            InsulatedCopperCable => "Insulated Copper Cable",
            Rubber => "Rubber",
            REBattery => "RE Battery",
            Generator => "Generator",
            Redstone => "Redstone",
            ElectronicCircuit => "Electronic Circuit",
            PulverizedCoal => "Pulverized Coal",
            Glass => "Glass",
            SolarPanel => "Solar Panel",
            LVTransformer => "LV Transformer",
            LVSolarArray => "Low Voltage Solar Array",
            _ => "#UNDEFINED_NAME",
        })
    }

    fn components(self) -> Option<Vec<(Item, f64)>> {
        match self {
            Furnace => Some(vec![(Cobblestone, 8.)]),
            CopperCable => Some(vec![(CopperBar, 0.5)]),
            InsulatedCopperCable => Some(vec![(CopperCable, 1.0), (Rubber, 1.0)]),
            IronFurnace => Some(vec![(Furnace, 1.), (IronBar, 5.)]),
            //RefinedIron => Some(vec![(IronBar, 1.)]),
            Generator => Some(vec![(IronFurnace, 1.), (RefinedIron, 3.), (REBattery, 1.)]),
            REBattery => Some(vec![
                (InsulatedCopperCable, 1.),
                (TinBar, 4.),
                (Redstone, 2.),
            ]),
            ElectronicCircuit => Some(vec![
                (InsulatedCopperCable, 6.),
                (RefinedIron, 1.),
                (Redstone, 2.),
            ]),
            SolarPanel => Some(vec![
                (Generator, 1.),
                (ElectronicCircuit, 2.),
                (PulverizedCoal, 3.),
                (Glass, 3.),
            ]),
            LVTransformer => Some(vec![
                (CopperBar, 3.),
                (WoodenPlanks, 4.),
                (InsulatedCopperCable, 2.),
            ]),
            LVSolarArray => Some(vec![(SolarPanel, 8.), (LVTransformer, 1.)]),
            GoldCable => Some(vec![(GoldBar, 0.25)]),
            InsulatedGoldCable => Some(vec![(GoldCable, 1.0), (Rubber, 1.0)]),
            TwiceInsulatedGoldCable => Some(vec![(InsulatedGoldCable, 1.0), (Rubber, 1.0)]),

            BasicMachineCasing => Some(vec![(RefinedIron, 8.)]),
            MVTransformer => Some(vec![
                (TwiceInsulatedGoldCable, 2.),
                (BasicMachineCasing, 1.),
            ]),
            MVSolarArray => Some(vec![(LVSolarArray, 8.), (MVTransformer, 1.)]),

            _ => None,
        }
    }

    fn cost(self, number: f64) -> Vec<(Self, f64)> {
        let mut elementaries = match self.components() {
            Some(items) => items
                .iter()
                .flat_map(|(item, n)| item.cost((*n) * number))
                .collect(),
            _ => vec![(self.clone(), number)],
        };
        elementaries.sort_by(|(item_a, _n_a), (item_b, _n_b)| item_a.partial_cmp(item_b).unwrap());

        elementaries
            .iter()
            .fold(vec![(elementaries[0].0, 0.)], |mut v, (item, n)| {
                let last = { v.len() - 1 };
                let (item0, _n0) = v[last];
                if item0 == *item {
                    v[last].1 += n;
                } else {
                    v.push((*item, *n));
                }
                v.clone()
            })
    }
}

fn stack(n: f64) -> String {
    let r = n % 64.;
    let s = (n / 64.).floor();
    if s >= 1. {
        format!("{} Stacks  + {}", s, r)
    } else {
        format!("             {}", r)
    }
}

fn main() {
    let meh = ElectronicCircuit;
    meh.cost(18.0)
        .iter()
        .for_each(|(item, n)| println!("{}: {}", item.to_string(), stack(*n)));
}
