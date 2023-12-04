-- test example for simulating a taxi booking app
GLOBAL = {
  max_sessions = 3,
  run_length = 10000,
  hb_interval = 30,
}

MACHINE = {
  {
    name = "open",
    default = true,
    probs = {login = 100},
    waitTime = 3,
    waitSpread = 17,
  },
  {
    name = "login",
    probs = {autologin=70,manual_login=30},
    waitTime = 10,
    waitSpread = 50,
  },
  {
    name = "autologin",
    probs = {login_success=70,autologin_fail=30},
    waitTime = 50,
    waitSpread = 25,
  },
  {
    name = "autologin_fail",
    probs = {login=100},
    waitTime=5,
    waitSpread=5,
  },
  {
    name = "manual_login",
    probs = {login_success=60,login_fail=40},
    waitTime=30,
    waitSpread=30,
  },
  {
    name = "login_success",
    probs = {home=100},
    waitTime=5,
  },
  {
    name = "login_fail",
    probs = {login=100},
    waitTime=2,
    waitSpread=5,
  },

  {
    name = "home",
    probs = {search_ride=80,closed=20},
    waitTime= 45,
    waitSpread = 15,
  },
  {
    name = "search_ride",
    probs = {ride_accepted=50,search_failed=50},
    waitTime=240,
    waitSpread=300,
  },
  {
    name = "search_failed",
    probs = {home=100},
    waitTime=5,
  },

  {
    name = "ride_accepted",
    probs = {driver_cancelled=30,ride_cancelled=5,ride_arrived=65},
    waitTime = 360,
    waitSpread = 360,
  },
  {
    name = "ride_cancelled",
    probs = {home=100},
    waitTime=5,
  },
  {
    name = "driver_cancelled",
    probs = {home=100},
    waitTime = 5,
  },


  {
    name = "ride_arrived",
    probs = {trip_started=95,trip_cancelled=5},
    waitTime = 60,
    waitSpread = 60,
  },
  {
    name = "trip_started",
    probs = {trip_finished=998,trip_cancelled=2},
    waitTime = 600,
    waitSpread = 1200,
  },
  {
    name = "trip_cancelled",
    probs = {home=100},
    waitTime = 5,
  },
  {
    name = "trip_finished",
    probs = {transaction_prompt=100},
    waitTime = 10,
    waitSpread = 10,
  },


  {
    name = "transaction_prompt",
    probs = {transaction_sucess=80,transaction_fail=20},
    waitTime = 120,
    waitSpread = 30,
  },
  {
    name = "transaction_sucess",
    probs = {home=5,closed=95},
    waitTime=5,
  },
  {
    name = "transaction_fail",
    probs = {payment_backlog=100},
    waitTime =5,
  },
  {
    name = "payment_backlog",
    probs = {home=5,closed=95},
    waitTime=5,
  },
  {
    name = "closed",
    probs = {},
    final = true,
  }

}

-- experimental stuff
function SESSION_MANAGER(t)
  if 0 < t and t < 1000 then
      return 2
   elseif 1000 < t and t < 2000 then
      return 5
    else
      return 1
   end
end


