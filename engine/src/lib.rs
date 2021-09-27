/*! engine for old wumpus game */

use anyhow::Result;
use askama::Template;
use getset::{CopyGetters, Getters};
use log::debug;
use narthex_engine_trait::{
    ActionTrait, ConfigTrait, EngineTrait, Event, InterfaceType, ResponseTrait,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumString;
/** we might not need a config but we need to declare one  */
#[derive(Default, Deserialize, Debug)]
pub struct Config {}
impl ConfigTrait for Config {
    fn from_json(_json_str: &str) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }
}

/** An `Action`  is something that the [Engine] does. */
#[derive(Debug, Deserialize, EnumString, Clone)]
#[repr(C)]
pub enum Action {
    Move(u8),
    Shoot(Vec<u8>),
    ReStart,
    Instructions,
    Quit,
}
impl ActionTrait for Action {
    fn from_json(json_str: &str) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_str(json_str)?)
    }
}
/** An `Engine`  is the engine for the Game */
#[derive(Debug, Default)]
pub struct Engine {
    interface_type: InterfaceType,
    data: Data,
}
impl EngineTrait for Engine {
    type Action = Action;
    type Response = Response;
    type Config = Config;
    /// create a new [Engine].
    fn new(_config: &Self::Config, interface_type: InterfaceType) -> Result<Self> {
        debug!("creating new game engine");
        Ok(Self {
            interface_type,
            data: Data::new(),
            ..Engine::default()
        })
    }
    /** `initial_html` provides the initial HTML. */
    fn initial_html(&mut self) -> Result<String> {
        let template = InitialTemplate {
            interface_type: self.interface_type,
            ..Default::default()
        };
        Ok(template.render()?)
    }
    /** `execute` executes the user command ([Action]) and returns a [Response]. */
    fn execute(&mut self, action: Action) -> Result<Response> {
        debug!("executing {:?}...", &action);
        self.data.msgs.clear();
        match action {
            Action::Instructions => {
                self.data.show_instructions();
                Ok(self.data.create_response())
            }
            Action::Move(cave) => {
                self.data.move_to(cave);
                debug!("moved");
                Ok(self.data.create_response())
            }
            Action::Shoot(path) => {
                self.data.shoot_arrow(path);
                debug!("arrow shot");
                Ok(self.data.create_response())
            }
            Action::ReStart => {
                self.data.renew();
                Ok(self.data.create_response())
            }
            Action::Quit => Ok(Response {
                shutdown_required: true,
                ..Response::default()
            }),
        }
    }
    fn handle_event(&mut self, event: &Event) -> Result<Self::Response> {
        match event {
            Event::Create => {
                /* do nothing  */
                Ok(Response::default())
            }
            Event::SaveInstanceState => {
                /* TO DO */
                Ok(Response::default())
            }
            _ => {
                debug!("event ignored {:?}", &event);
                Ok(Response::default())
            } // TODO add
        }
    }

    /// interface type
    fn get_interface_type(&self) -> InterfaceType {
        self.interface_type
    }
}
/** template for generating the initial panel */
#[derive(Template, Default)]
#[template(path = "initial.html")]
struct InitialTemplate {
    interface_type: InterfaceType,
}
/** A `Response` is the response of the [Engine] to the webview. */
#[derive(Debug, Default, Getters, CopyGetters, Clone, Serialize)]
#[repr(C)]
pub struct Response {
    /// whether the [Engine] and the main program should shut down.
    #[getset(get_copy = "pub")]
    shutdown_required: bool,
    msgs: String,
    tunnels: [u8; 3],
}
impl ResponseTrait for Response {
    fn shutdown_required(&self) -> bool {
        self.shutdown_required
    }
}
impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Response)",)
    }
}
// based on the original PROGRAM LISTING
/* 0010 */  //  HUNT THE WUMPUS
/* 0015 */  // :  BY GREGORY YOB

/* 0170 */
fn rand20() -> u8 {
    (20.0 * rand::random::<f64>()) as u8 + 1
}
/* 0170 */
fn rand3() -> u8 {
    (3.0 * rand::random::<f64>()) as u8 + 1
}
/* 0170 */
fn rand4() -> u8 {
    (4.0 * rand::random::<f64>()) as u8 + 1
}
#[derive(Debug, Default)]
struct Data {
    s: [[u8; 3]; 20],
    l: [u8; 6],
    m: [u8; 6],
    ll: u8,
    a: i8,
    f: i8,
    msgs: Vec<String>,
    tunnels: [u8; 3],
}
impl Data {
    pub fn new() -> Self {
        /* 0200 */
        // LOCATE L ARRAY ITEMS
        /* 0210 */  // 1-YOU,2-WUMPUS,3&4-PITS,5&6-BATS
        let mut new_data = Self{
            /* 0068 */  //  SET UP CAVE (DODECAHEDRAL NODE LIST)
            s: /* 0130 */  [ [2,5,8],[1,3,10],[2,4,12],[3,5,14],[1,4,6],[
                /* 0140 */    5,7,15],[6,8,17],[1,7,9],[8,10,18],[2,9,11], [                /* 0150 */    10,12,19],[3,11,13],[12,14,20],[4,13,15],[6,14,16],[
                /* 0160 */   15,17,20],[7,16,18],[9,17,19],[11,18,20],[13,16,19]],
        msgs: vec!["HUNT THE WUMPUS".to_string()],
        // f: 0,
        // tunnels: [0; 3],
        ..Default::default()
    };
        new_data.renew();
        new_data
    }
    fn renew(&mut self) {
        let mut good = false;
        let mut lm = [0u8; 6];
        while !good {
            good = true;
            for lmx in &mut lm {
                *lmx = rand20();
            }

            /* 0280 */
            // CHECK FOR CROSSOVERS (IE L(1)=L(2),ETC)
            for i in 0..6 {
                for j in 0..6 {
                    if i != j && lm[i] == lm[j] {
                        good = false;
                    }
                }
            }
        }
        self.l = lm;
        self.m = lm;
        self.ll = lm[0];
        self.f = 0;
        /* 0350 */
        // SET# ARROWS
        self.a = 5;
    }

    fn create_response(&mut self) -> Response {
        /* 0380 */
        // HAZARD WARNINGS & LOCATION
        /* 0390 */
        if self.f == 0 {
            self.print_warnings();
        } else if self.f < 0 {
            /* 0510 */
            // LOSE
            /* 0520 */
            self.say("HA HA HA - YOU LOSE!");
        } else {
            /* 0540 */
            // WIN
            /* 0550 */
            self.say("HEE HEE HEE - THE WUMPUS'LL GETCHA NEXT TIME!!");
        }
        Response {
            msgs: self.msgs.join("<br/>"),
            tunnels: self.tunnels,
            ..Response::default()
        }
    }
    fn show_instructions(&mut self) {
        self.say(
            " WELCOME TO 'HUNT THE WUMPUS'<br/>
THE WUMPUS LIVES IN A CAVE OF 20 ROOMS. EACH ROOM
HAS 3 TUNNELS LEADING TO OTHER ROOMS. (LOOK AT A
DODECAHEDRON TO SEE HOW THIS WORKS-IF YOU DON'T KNOW
WHAT A DODECAHEDRON IS, ASK SOMEONE)<br/>
<br/>
HAZARDS:<br/>
BOTTOMLESS PITS - TWO ROOMS HAVE BOTTOMLESS PITS IN THEM
IF YOU GO THERE, YOU FALL INTO THE PIT (& LOSE!)<br/>
SUPER BATS - TWO OTHER ROOMS HAVE SUPER BATS. IF YOU
GO THERE, A BAT GRABS YOU AND TAKES YOU TO SOME OTHER
ROOM AT RANDOM. (WHICH MIGHT BE TROUBLESOME)<br/>
<br/>
WUMPUS:<br/>
THE WUMPUS IS NOT BOTHERED BY THE HAZARDS (HE HAS SUCKER
FEET AND IS TOO BIG FOR A BAT TO LIFT). USUALLY
HE IS ASLEEP. TWO THINGS WAKE HIM UP: YOUR ENTERING
HIS ROOM OR YOUR SHOOTING AN ARROW.
IF THE WUMPUS WAKES, HE MOVES (P=.75) ONE ROOM
OR STAYS STILL (P=.25). AFTER THAT, IF HE IS WHERE YOU
ARE, HE EATS YOU UP (& YOU LOSE!)<br/>
<br/>
YOU:<br/>
EACH TURN YOU MAY MOVE OR SHOOT A CROOKED ARROW<br/>
MOVING: YOU CAN GO ONE ROOM (THRU ONE TUNNEL)<br/>
ARROWS: YOU HAVE 5 ARROWS. YOU LOSE WHEN YOU RUN OUT.
EACH ARROW CAN GO FROM 1 TO 5 ROOMS. YOU AIM BY TELLING
THE COMPUTER THE ROOM#S YOU WANT THE ARROW TO GO TO.
IF THE ARROW CAN'T GO THAT WAY (IE NO TUNNEL) IT MOVES
AT RAMDOM TO THE NEXT ROOM.
IF THE ARROW HITS THE WUMPUS, YOU WIN.
IF THE ARROW HITS YOU, YOU LOSE.<br/>
<br/>
WARNINGS:<br/>
WHEN YOU ARE ONE ROOM AWAY FROM WUMPUS OR HAZARD,
THE COMPUTER SAYS:<br/>
WUMPUS- 'I SMELL A WUMPUS'<br/>
BAT - 'BATS NEARBY'<br/>
PIT - 'I FEEL A DRAFT'<br/>",
        );
    }
    // /* 0400 */  // MOVE OR SHOOT
    // /* 0410 */  GOSUB 2500
    // /* 0420 */  GOTO O OF 440,480
    // /* 0430 */  // SHOOT
    // /* 0440 */  GOSUB 3000
    // /* 0450 */  IF F=0 THEN 390
    // /* 0460 */  GOTO 500
    // /* 0470 */  // MOVE
    // /* 0480 */  GOSUB 4000
    // /* 0560 */   FOR J=1 TO 6
    // /* 0570 */   L(J)=M(J)
    // /* 0580 */   NEXT J
    // /* 0590 */   self.msgs.push( "SAME SET-UP (Y-N)".to_string());;
    // /* 0600 */  INPUT I$
    // /* 0610 */  IF I$#"Y" THEN 240
    // /* 0620 */  GOTO 360
    // /* 1000 */  // INSTRUCTIONS
    // /* 1010 */
    // /* 1410 */  RETURN
    /* 2000 */  // PRINT LOCATION & HAZARD WARNINGS
    fn print_warnings(&mut self) {
        /* 2020 */
        for j in 1..5 {
            /* 2030 */
            for k in 0..2 {
                if self.s[self.l[0] as usize - 1][k] == self.l[j] {
                    match j - 1 {
                        /* 2060 */
                        0 => {
                            self.say("I SMELL A WUMPUS!");
                        }
                        /* 2080 */
                        1 | 2 => {
                            self.say("I FEEL A DRAFT");
                        }
                        /* 2100 */
                        3 | 4 => {
                            self.say("BATS NEARBY!");
                        }
                        _ => {} /* 2090 */
                    }
                }
            }
        }
        /* 2130 */
        self.say(&format!("YOU ARE IN ROOM {}", self.l[0]));
        /* 2140 */
        let ss = self.s[self.l[0] as usize - 1];
        self.say(&format!("TUNNELS LEAD TO {}, {}, {}", ss[0], ss[1], ss[2]));
        self.tunnels = [ss[0], ss[1], ss[2]];
        /* 2160 */
    }
    // /* 3000 */  // ARROW ROUTINE
    // /* 3010 */  F=0
    // /* 3020 */  // PATH OF ARROW
    // /* 3030 */ let mut p = vec![];
    // /* 3040 COMEHERE */   self.msgs.push( "NO. OF ROOMS(1-5)".to_string());;
    // /* 3050 COMEHERE */  INPUT J9
    // /* 3060 */  IF J9<1 OR J9>5 THEN 3040
    // /* 3070 */   FOR K=1 TO J9
    // /* 3080 COMEHERE */    self.msgs.push( "ROOM #";
    // /* 3090 */   INPUT P(K)
    // /* 3095 */   IF K <= 2 THEN 3115
    // /* 3100 */   IF P(K) <> P(K-2) THEN 3115
    // /* 3105 */    self.msgs.push( "ARROWS AREN'T THAT CROOKED - TRY ANOTHER ROOM".to_string());
    // /* 3110 */   GOTO 3080
    // /* 3115 COMEHERE */   NEXT K
    /* 3120 */  // SHOOT ARROW
    fn shoot_arrow(&mut self, p: Vec<u8>) {
        /* 3130 */
        self.ll = self.l[0];
        /* 3140 */
        for k in 0..p.len() {
            let mut arrow_tunnel = false;
            /* 3150 */
            for k1 in 0..3 {
                if self.s[self.ll as usize - 1][k1] == p[k] {
                    arrow_tunnel = true;
                }
            }
            if !arrow_tunnel {
                /* 3180 */
                // NO TUNNEL FOR ARROW
                /* 3190 */
                self.ll = self.s[self.ll as usize - 1][rand3() as usize - 1];
                /* 3200 */
                // return;
            }
            /* 3100 */
            if k > 1 && p[k - 2] == p[k] {
                /* 3105 */
                self.say("ARROWS AREN'T THAT CROOKED");
                return;
            }
            self.check_arrow(&p, k);
            if self.f != 0 {
                return;
            }
        }
        /* 3210 */
        /* 3220 */
        self.say("MISSED");
        /* 3225 */
        self.ll = self.l[1];
        /* 3230 */
        // MOVE WUMPUS
        /* 3240 */
        self.move_wumpus();
        /* 3250 */
        // AMMO CHECK
        /* 3255 */
        self.a -= 1;
        /* 3260 */
        if self.a <= 0 {
            /* 3270 */
            self.f = -1;
            /* 3280 */
            return;
        }
    }
    fn check_arrow(&mut self, p: &Vec<u8>, k: usize) {
        /* 3290 */
        // SEE IF ARROW IS AT L(1) OR L(2)
        /* 3295 COMEHERE */
        self.ll = p[k];
        /* 3300 COMEHERE */
        if self.ll == self.l[1] {
            /* 3310 */
            self.say("AHA! YOU GOT THE WUMPUS!");
            /* 3320 */
            self.f = 1;
            return;
            /* 3330 */
        }
        /* 3340 */
        /* 3300 */
        if self.ll == self.l[0] {
            /* 3350 */
            self.say("OUCH! ARROW GOT YOU!");
            self.f = -1;
            return;
            /* 3360 */
        }
    }
    /* 3370 */
    // MOVE WUMPUS ROUTINE
    fn move_wumpus(&mut self) {
        /* 3380 */
        let k = rand4() as usize;
        debug!("moving wumpus from {} to #{}", self.l[1], &k);
        /* 3390 */
        if k != 4 {
            /* 3400 */
            self.l[1] = self.s[self.l[1] as usize - 1][k - 1];
        }
        /* 3410 */
        if self.l[1] == self.l[0] {
            /* 3420 */
            self.say("TSK TSK TSK- WUMPUS GOT YOU!");
            /* 3430 */
            self.f = -1;
        }
        /* 3440 */
        debug!("wumpus moved to room {}", self.l[1]);
    }
    /* 4000 */
    //  MOVE ROUTINE
    fn move_to(&mut self, l: u8) {
        debug!("moving to room {}", l);
        let mut lx = l;
        /* 4010 */
        self.f = 0;
        let mut ok = false;
        /* 4050 */
        for k in 0..3 {
            /* 4060 */
            //  CHECK IF LEGAL MOVE
            if self.s[self.l[0] as usize - 1][k as usize] == lx {
                ok = true;
            }
            /* 4080 */
        }
        if !ok {
            /* 4090 */
            if lx != self.l[0] {
                /* 4100 */
                self.say("NOT POSSIBLE -");
            }
            /* 4110 */
            return;
        }
        loop {
            /* 4120 */
            // CHECK FOR HAZARDS
            /* 4130 */
            self.l[0] = lx;
            /* 4140 */
            // WUMPUS
            /* 4150 */
            if lx == self.l[1] {
                /* 4160 */
                self.say("...OOPS! BUMPED A WUMPUS!");
                /* 4170 */
                // MOVE WUMPUS
                /* 4180 */
                self.move_wumpus();
            }
            /* 4190 */
            if self.f != 0 {
                return;
            }
            /* 4210 */
            // PIT
            /* 4220 */
            if lx == self.l[2] || lx == self.l[3] {
                /* 4230 */
                self.say("YYYIIIIEEEE . . . FELL IN PIT");
                /* 4240 */
                self.f = -1;
                /* 4250 */
                return;
            }
            /* 4260 */
            // BATS
            /* 4270 */
            if lx != self.l[4] && lx != self.l[5] {
                break;
            }
            /* 4280 */
            self.say("ZAP--SUPER BAT SNATCH! ELSEWHEREVILLE FOR YOU!");
            /* 4290 */
            lx = rand20();
            /* 4310 */
        }
        /* 5000 */
    }
    fn say(&mut self, msg: &str) {
        debug!("saying: {}", &msg);
        self.msgs.push(msg.to_string());
    }
}
