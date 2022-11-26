// how do we represent energy that is used in the body?
struct Energy {
    // wavelength of the energy
    wavelength: f32,
    // intensity of the energy
    vibrationIntensity: f32,
}

struct Memory {
    // activation threshold
    activationThreshold: f64,
    // voltage
    voltage: f64,
}

struct Sense {
    // linked list of memories- or a graph? should be a neural network but this is mvp
    memories: Vec<Memory>, // basically an array of objects
    // size of brain used for memories
    amountOfMemory: u32,
}

impl Sense {
    fn sense(&self) -> bool {
        true
    }
    fn filter(&self) -> bool {
        true
    }
    fn store(&self) -> bool {
        true
    }
    fn access(&self) -> bool {
        true
    }
}

struct Catch {}
impl Catch {
    fn catch(&self) -> bool {
        true
    }
}

struct Consume {}
impl Consume {
    fn consume(&self) -> bool {
        // calls catch, then chew, then swallow, then breakdown, then transportEnergy, then useEnergy, then storeEnergy
        true
    }
}

struct Digest {}
impl Digest {
    fn breakDown(&self) -> bool {
        true
    }
    fn transportEnergy(&self) -> bool {
        true
    }
    fn useEnergy(&self) -> bool {
        true
    }
    fn storeEnergy(&self) -> bool {
        true
    }
}

struct Replicate {}
impl Replicate {
    fn replicate(&self) -> bool {
        true
    }
}

struct Defend {}
impl Defend {
    fn defend(&self) -> bool {
        true
    }
}

struct Move {}
impl Move {
    fn move_(&self) -> bool {
        true
    }
}

struct Communicate {}
impl Communicate {
    fn communicate(&self) -> bool {
        true
    }
}

struct Repair {}
impl Repair {
    fn repair(&self) -> bool {
        true
    }
}

struct Life {
    sense: Sense,
    catch: Catch,
    consume: Consume,
    digest: Digest,
    replicate: Replicate,
    defend: Defend,
    move_: Move,
    communicate: Communicate
}

// Path: main.rs
fn main() {
    println!("in main");
    let life = Life {
        sense: Sense {
            memories: Vec::new(),
            amountOfMemory: 0,
        },
        catch: Catch {},
        consume: Consume {},
        digest: Digest {},
        replicate: Replicate {},
        defend: Defend {},
        move_: Move {},
        communicate: Communicate {},
    };
}