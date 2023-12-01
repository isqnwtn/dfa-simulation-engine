GLOBAL = {
  max_sessions = 3,
  run_length = 20,
  session_manager = function (t)
   if 0 < t < 1000 then
      return {session_count = 2}
   elseif 1000 < t < 2000 then
      return {session_count = 5}
    else
      return {session_count = 1}
   end
  end
}

MACHINE = {
  {
    name = "inactive",
    default = true,
    probs = {playing = 1.0},
    waitTime = 90,
  },
  {
    name = "playing",
    probs = {inactive = 0.2,paused = 0.7, stopped= 0.1},
    waitTime = 200,
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
