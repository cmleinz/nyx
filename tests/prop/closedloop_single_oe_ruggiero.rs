extern crate hifitime;
extern crate nalgebra as na;
extern crate nyx_space as nyx;

use self::hifitime::{Epoch, SECONDS_PER_DAY};
use self::nyx::celestia::{bodies, Cosm, Geoid, State};
use self::nyx::dynamics::celestial::CelestialDynamics;
use self::nyx::dynamics::propulsion::{Propulsion, Thruster};
use self::nyx::dynamics::spacecraft::Spacecraft;
use self::nyx::dynamics::thrustctrl::{Achieve, Ruggiero};
use self::nyx::dynamics::Dynamics;
use self::nyx::propagators::{PropOpts, Propagator, RK4Fixed};

#[test]
fn rugg_sma() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let orbit = State::<Geoid>::from_keplerian(24396.0, 0.0, 0.0, 0.0, 0.0, 0.0, start_time, earth);

    let prop_time = 45.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Sma {
        target: 42164.0,
        tol: 1.0,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 21.0).abs() < 1.0);
}

#[test]
fn rugg_sma_decr() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let orbit = State::<Geoid>::from_keplerian(42164.0, 0.0, 0.0, 0.0, 0.0, 0.0, start_time, earth);

    let prop_time = 45.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Sma {
        target: 24396.0,
        tol: 1.0,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 21.0).abs() < 1.0);
}

#[test]
fn rugg_inc() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 350.0;

    let orbit = State::<Geoid>::from_keplerian(sma, 0.001, 46.0, 1.0, 1.0, 1.0, start_time, earth);

    let prop_time = 55.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Inc {
        target: 51.6,
        tol: 5e-3,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 25.0).abs() < 1.0);
}

#[test]
fn rugg_inc_decr() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 350.0;

    let orbit = State::<Geoid>::from_keplerian(sma, 0.001, 51.6, 1.0, 1.0, 1.0, start_time, earth);

    let prop_time = 55.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Inc {
        target: 46.0,
        tol: 5e-3,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 25.0).abs() < 1.0);
}

#[test]
fn rugg_ecc() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 9000.0;

    let orbit = State::<Geoid>::from_keplerian(sma, 0.01, 98.7, 0.0, 1.0, 1.0, start_time, earth);

    let prop_time = 30.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Ecc {
        target: 0.15,
        tol: 5e-5,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 14.0).abs() < 1.0);
}

#[test]
fn rugg_ecc_decr() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 9000.0;

    let orbit = State::<Geoid>::from_keplerian(sma, 0.15, 98.7, 0.0, 1.0, 1.0, start_time, earth);

    let prop_time = 30.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Ecc {
        target: 0.01,
        tol: 5e-5,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 14.0).abs() < 1.0);
}

#[test]
fn rugg_aop() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 900.0;

    // Note that AOP computation requires the orbit to not be equatorial or circular, hence the non-zero ECC and INC.
    let orbit = State::<Geoid>::from_keplerian(sma, 5e-5, 5e-3, 0.0, 178.0, 0.0, start_time, earth);

    // This is a very quick change because we aren't using the Ruggiero formulation for AOP change and benefit both in-plane and out of plane control.
    let prop_time = 2650.0;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Aop {
        target: 183.0,
        tol: 5e-3,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 0.014).abs() < 1.0);
}

#[test]
fn rugg_aop_decr() {
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2020, 1, 1);

    let sma = earth.equatorial_radius + 900.0;

    // Note that AOP computation requires the orbit to not be equatorial or circular, hence the non-zero ECC and INC.
    let orbit = State::<Geoid>::from_keplerian(sma, 5e-5, 5e-3, 0.0, 183.0, 0.0, start_time, earth);

    let prop_time = 2650.0;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Aop {
        target: 178.0,
        tol: 5e-3,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 0.014).abs() < 1.0);
}

#[test]
#[ignore]
fn rugg_raan() {
    // BUG: https://gitlab.com/chrisrabotin/nyx/issues/83
    let cosm = Cosm::from_xb("./de438s");
    let earth = cosm.geoid_from_id(bodies::EARTH);

    let start_time = Epoch::from_gregorian_tai_at_midnight(2017, 1, 1);

    let sma = earth.equatorial_radius + 798.0;

    let orbit =
        State::<Geoid>::from_keplerian(sma, 0.00125, 98.57, 0.0, 1.0, 0.0, start_time, earth);

    let prop_time = 49.0 * SECONDS_PER_DAY;

    // Define the dynamics
    let mut dynamics = CelestialDynamics::two_body(orbit);

    // Define the thruster
    let lowt = vec![Thruster {
        thrust: 89e-3,
        isp: 1650.0,
    }];

    // Define the objectives
    let objectives = vec![Achieve::Raan {
        target: 5.0,
        tol: 5e-3,
    }];

    let mut ruggiero = Ruggiero::new(objectives, orbit);

    let fuel_mass = 67.0;
    let dry_mass = 300.0;

    let mut prop_subsys = Propulsion::new(&mut ruggiero, lowt, true);

    let mut sc = Spacecraft::with_prop(&mut dynamics, &mut prop_subsys, dry_mass, fuel_mass);
    println!("{:o}", orbit);

    let mut prop = Propagator::new::<RK4Fixed>(&mut sc, &PropOpts::with_fixed_step(10.0));
    prop.until_time_elapsed(prop_time);

    let final_state = prop.dynamics.celestial.state();
    let fuel_usage = fuel_mass - sc.fuel_mass;
    println!("{:o}", final_state);
    println!("fuel usage: {:.3} kg", fuel_usage);

    assert!(ruggiero.achieved(&final_state), "objective not achieved");
    assert!((fuel_usage - 48.0).abs() < 1.0);
}