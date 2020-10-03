// QUESTION

// A child is playing with a ball on the nth floor of a tall building. The height of this floor, h, is known.

// He drops the ball out of the window. The ball bounces (for example), to two-thirds of its height (a bounce of 0.66).

// His mother looks out of a window 1.5 meters from the ground.

// How many times will the mother see the ball pass in front of her window (including when it's falling and bouncing?

// Three conditions must be met for a valid experiment:
// Float parameter "h" in meters must be greater than 0
// Float parameter "bounce" must be greater than 0 and less than 1
// Float parameter "window" must be less than h.
// If all three conditions above are fulfilled, return a positive integer, otherwise return -1.

// Note:
// The ball can only be seen if the height of the rebounding ball is strictly greater than the window parameter.

// Example:
// - h = 3, bounce = 0.66, window = 1.5, result is 3

// - h = 3, bounce = 1, window = 1.5, result is -1 

// (Condition 2) not fulfilled).

// BREAKDOWN

// There are three vars to keep track of this time, and one return, the count of the number of balls seen.
// The count 'starts' at one, since the person sees the ball the first time the ball falls. After here, every bounce that goes 'past'
// the window will count as count+1, as the person will see the ball once as it goes up, and again as it goes down.
// when the ball hits the ground, the bounce coefficient will multiply against the existing bounce_height var, as the ball loses kinetic energy.
// when the ball no longer goes past the window, stop counting and return the count value.

fn main() {
}

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h > 0.0 && bounce > 0.0 && bounce < 1. && window < h{

        let mut count:i32 = 1;
        let mut bounce_height:f64 = h * bounce;
        while bounce_height > window {
            count +=2;
            bounce_height = bounce_height * bounce;
        }
        return count;
    }
    else{
        return -1
    }
}

fn testequal(h: f64,  bounce: f64,  window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {

    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
  
}