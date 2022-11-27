/// @notice A program that represents a life entity with memory states and functions to interact with the world.
pub const IS_GNU: u8 = 0x10;

/// @notice State constants
struct Constants {
    activationThreshold: f64,
    isConscious: bool,
}

impl Default for Constants {
    fn default() -> Constants {
        Constants {
            activationThreshold: 0.7,
            isConscious: true,
        }
    }
}

/// @notice calculates the velocity of an energy byte snippet for a lifeform's sense
/// @param period
/// @param amplitude
/// @returns velocity
fn velocity(period: f32, wavelength: f32) -> f32 {
    // frequency = 1 / unit of time (period) in seconds (measured in time units (seconds), frequency is measured in hertz (Hz))
    let frequency = 1.0 / period;
    // velocity (intensity) = frequency * wavelength
    let velocity = frequency * wavelength;

    return velocity as f32;
}

/// @notice Represent energy that triggers action
// ***final number will be a combination of amplitude and velocity
// how can we measure the amount of energy in bytes?
struct Energy {
    // wavelength of the energy, measured by period- the higher the wavelength, the higher the energy
    period: f32,
    // intensity of the energy- measured in amplitude
    amplitude: f32,
    // wavelength - distance between two consecutive crests or troughs (measured in nano meters (nm))
    wavelength: f32,
    // weighted energy to trigger activation energy threshold
    velocity: f32
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
    fn sense(period: f32, wavelength: f32) -> bool {
        // calculate velocity
        let velocity = velocity(period, wavelength);
        // check if velocity is greater than activation threshold
        // if velocity
            // check if the specific velocity is in the memories

                // if it is, return action

            // else add to memories
                
                // return new action
        
            // else ignore, store to memories

        true
    }
    fn filter(&self) -> bool {
        true
    }
    fn store(&self) -> bool {
        true
    }
    fn access(&self) -> bool {
        // 
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
            totalMemorySize: 0,
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