-- NOTE: Some of the functionalities mentioned here are still wip, at present we can handle 
-- state changes based on probability, future plans involves evaluation of functions that 
-- determine and modify the probability distribution of transitions from each state.
--
GLOBAL = {
  max_sessions = 3,
  run_length = 60,
  hb_interval = 30,
}

GLOBAL_STATE = {
  some_global_state_data = "something",
  sessions_in_rollup = 5
}

function GLOBAL_STATE_MANAGER(time,st)
  if (0<time) and (time<1000) then
    st.sessions_in_rollup = 100
  elseif (1000<time) and (time<3000) then
    st.sessions_in_rollup = 500
  elseif (3000<time) and (time<5000) then
    st.sessions_in_rollup = 1000
  elseif (5000<time) and (time<10000) then
    st.sessions_in_rollup = 750
  else
    st.sessions_in_rollup = 250
  end
 return st
end

MACHINE = {
  machine_vars = {
    runtime=0,
    max_runtime=200,
    is_minimized = false,
  },
  states = {
    {
      name = "inactive",
      default = true,
      probs = {playing = 100},
      waitTime = 90,
    },
    {
      name = "playing",
      probs = {buffering = 10,paused = 30, stopped= 10, playing=50},
      waitTime = 300,
      waitSpread = 50,
      updateFunction = function (time,machine_meta,machine_vars,_global_state)
        local new_runtime = time - machine_meta.start_time
        machine_vars.runtime = new_runtime
        local new_probs = {
          buffering = 10,
          paused = 30,
          stopped = 10 + 80*(new_runtime/machine_vars.max_runtime),
          playing = 50 - 45*(new_runtime/machine_vars.max_runtime)
        }
       return {machine_vars=machine_vars,probs=new_probs,waitTime=300,waitSpread=50}
      end
    },
    {
      name = "paused",
      probs = {playing = 90, stopped=10},
      waitTime = 30,
      waitSpread = 120,
    },
    {
      name = "buffering",
      probs = {playing=80,paused=20},
      waitTime = 5,
      waitSpread = 10,
    },
    {
      name = "stopped",
      probs = {},
      final = true,
    }
  }
}
