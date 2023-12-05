-- First test example for simulating a video player

-- Global variables used by the simulator
GLOBAL = {
  -- max sessions in rollup
  max_sessions = 3,
  -- how many transitions to compute
  run_length = 10000,
  -- whats the interval in which we should generate heartbeat
  hb_interval = 30,
}

-- * Ability to define all the aspects of PFA
-- list all states with state specific details
MACHINE = {
  {
    -- state name
    name = "inactive",
    -- is this the starting state
    default = true,
    -- probability distribution of transitions
    probs = {playing = 1.0},
    -- how long should we spend on this state
    waitTime = 90,
  },
  {
    name = "playing",
    probs = {inactive = 0.2,paused = 0.7, stopped= 0.1},
    waitTime = 200,
    -- used to add probability to the wait time 
    waitSpread = 50,
  },
  {
    name = "paused",
    probs = {playing = 1.0},
    waitTime = 50,
    waitSpread = 25,
  },
  {
    name = "stopped",
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


