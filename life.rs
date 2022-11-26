use bytes::Bytes;

/// @notice Represent energy that triggers action
// how can we measure the amount of energy in bytes?
struct Energy {
    // wavelength of the energy, measured by period- the higher the wavelength, the higher the energy
    period: f32,
    // intensity of the energy- measured in amplitude
    amplitude: f32,
    // weighted energy to trigger activation energy threshold
    energyWeight: f32,

    // intensity of the energy = amplitude / period
    // higher energy waves will be more intense, with shorter periods, but higher amplitudes.
}

/// @notice A single memory
struct Memory {
    // activation threshold
    activationThreshold: f64,
    // voltage
    voltage: f64,
    // size of memory (in bytes)
    size: u64,
}

/// @notice A Sense- should this be broken up in to the different senses? But each sense is just a wavelength
struct Sense {
    // linked list of memories- or a graph? should be a neural network but this is mvp
    memories: Vec<Memory>, // basically an array of objects
    // size of brain used for memories
    amountOfMemory: u32,
    // total size of memories in bytes
    totalMemorySize: u32,
}

impl Sense {
    /// @notice Senses new information
    /// @param new Energy- wavelength, vibrationIntensity (vibrationIntensity should probably be the integer that determines the size of the memory)
    fn sense(&self) -> bool {
        // check if the wavelength and vibrationIntensity matrix (wavelength x vibrationIntensity) is in the memories
            // if yes, and same number, recall what action to take
                // check if the vibrationIntensity is greater than the activationThreshold
            // if not, create new memory and add it to the memories array (add new action to take- how do we decide what action to take?)

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