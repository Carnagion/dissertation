// Represents a window of time such that `earliest <= target <= latest`
tuple TimeWindow {
  	int earliest;
  	int target;
  	int latest;
};

// Maximum time at which any operation can be scheduled
int maxTime = ...;
assert ValidMaxTime:
	maxTime >= 0;

// Available time range for scheduling all flights
range Time = 1..maxTime;

// Maximum allowed holdover time
int maxHoldoverDur = ...;
assert ValidMaxHoldover:
	maxHoldoverDur >= 0;

// Maximum allowed slack time for 
int maxSlackDur = ...;
assert ValidMaxSlack:
	maxSlackDur >= 0;

// Flight kind constants - departure or arrival
string departure = "departure";
string arrival = "arrival";

// Number of flights to schedule
int nbOfFlights = ...;
assert ValidFlightCount:
	nbOfFlights >= 0;

// Range of flight indices
range Flights = 1..nbOfFlights;

// Represents a departure or arrival to be scheduled
tuple Flight {
 	// NOTE: Always must be one of "departure" or "arrival"
  	string kind;
  	
  	TimeWindow window;
  	
  	// NOTE: For departures only
  	int pushBackDur;
  	int taxiDeIceDur;
  	int deIceDur;
  	int taxiOutDur;
  	int lineUpDur;
  	
  	// NOTE: For arrivals only
  	int taxiInDur;
};

// Set of flights to schedule
Flight flights[i in Flights] = ...;
assert ValidFlightData:
	forall (i in Flights)
	  	flights[i].kind in { departure, arrival }
	  	&& (0 <= flights[i].window.earliest
	  		<= flights[i].window.target
  			<= flights[i].window.latest
  			<= maxTime)
	  	&& (flights[i].kind == departure
	  		=> (flights[i].pushBackDur >= 0
	  			&& flights[i].taxiDeIceDur >= 0
	  			&& flights[i].deIceDur >= 0
	  			&& flights[i].taxiOutDur >= 0
	  			&& flights[i].lineUpDur >= 0
	  			&& (flights[i].window.earliest
	  				- (flights[i].pushBackDur
	  					+ flights[i].taxiDeIceDur
	  					+ flights[i].deIceDur
	  					+ flights[i].taxiOutDur
	  					+ flights[i].lineUpDur
	  					+ maxSlackDur)) in Time)
				&& (flights[i].taxiOutDur + flights[i].lineUpDur <= maxHoldoverDur))
		&& (flights[i].kind == arrival
			=> (flights[i].taxiInDur >= 0
				&& (flights[i].window.latest + flights[i].taxiInDur) in Time));

// Represents a departure to be scheduled
tuple Dep {
  	TimeWindow ctot;
  	
  	int pushBackDur;
  	int taxiDeIceDur;
  	int deIceDur;
  	int taxiOutDur;
  	int lineUpDur;
};

// Set of departure indices
setof(int) Departures = { i | i in Flights: flights[i].kind == departure };

// Set of departures to schedule
Dep deps[i in Departures] = <
	flights[i].window,
	flights[i].pushBackDur,
	flights[i].taxiDeIceDur,
	flights[i].deIceDur,
	flights[i].taxiOutDur,
	flights[i].lineUpDur>;

// Represents an arrival to be scheduled
tuple Arr {
  	TimeWindow window;
  	
  	int taxiInDur;
};

// Set of arrival indices
setof(int) Arrivals = { i | i in Flights: flights[i].kind == arrival };

// Set of arrivals to schedule
Arr arrs[i in Arrivals] = <
	flights[i].window,
	flights[i].taxiInDur>;

// Separations between each flight, where `sep[i, j]` is the separation between
// flights `i` and `j` assuming that `j` departs or arrives after `i`
int sep[i in Flights, j in Flights] = ...;
assert ValidSeparations:
	forall (i, j in Flights)
		sep[i, j] >= 0;

// Set of possible departure times for a departure `i`
// TODO: Factor in allowance before and after CTOT window
setof(int) PossibleDepartureTimes[i in Departures] = { t | t in Time:
	t in deps[i].ctot.earliest..deps[i].ctot.latest };

// Set of possible arrival times for an arrival `i`
setof(int) PossibleArrivalTimes[i in Arrivals] = { t | t in Time:
	t in arrs[i].window.earliest..arrs[i].window.latest };
	
// Set of possible departure or arrival times for a flight `i`
setof(int) PossibleSchedTimes[i in Flights] = { t | t in Time:
	(flights[i].kind == departure && t in PossibleDepartureTimes[i])
	|| flights[i].kind == arrival && t in PossibleArrivalTimes[i] };

tuple FlightSched {
  	int flight;
  	int time;
};

// Set of flights and scheduled times for those flights such that `t` is a possible
// time for scheduling the arrival or departure of `i`
setof(FlightSched) PossibleScheds = { <i, t> | i in Flights, t in PossibleSchedTimes[i] };

// Whether a flight `i` departs or arrives at time `t`
dvar boolean isSchedAt[<i, t> in PossibleScheds];

// Whether a departure `i` is dropped
dvar boolean isDropped[i in Departures];

// Set of possible de-icing times for a departure `i`
setof(int+) PossibleDeIceTimes[i in Departures] = { t | t in Time:
	t in (deps[i].ctot.earliest - (deps[i].lineUpDur + deps[i].taxiOutDur + deps[i].deIceDur + maxSlackDur))
		..(deps[i].ctot.latest - (deps[i].lineUpDur + deps[i].taxiOutDur + deps[i].deIceDur)) };

tuple DeIceSched {
  	int dep;
  	int time;
};

// Set of departures and de-icing times for those departures such that `t` is a possible
// de-icing time for departure `i`
setof(DeIceSched) PossibleDeIceScheds = { <i, t> | i in Departures, t in PossibleDeIceTimes[i] };

// Whether a departure `i` de-ices at time `t`
dvar boolean isDeIceAt[<i, t> in PossibleDeIceScheds];

// TODO: Figure out a suitable cost for delays
dexpr int delay[<i, t> in PossibleScheds] = isSchedAt[<i, t>] * (
	(t in flights[i].window.earliest..flights[i].window.target - 1) * ftoi(pow(flights[i].window.target - t, 2))
	+ (t in flights[i].window.target + 1..flights[i].window.latest) * ftoi(pow(t - flights[i].window.target, 2))
);

// TODO: Figure out suitable cost for dropping a departure
dexpr int drop[i in Departures] = isDropped[i] * 1000;

tuple DepDeIceSched {
  	int dep;
  	int deIceTime;
  	int depTime;
};

// Set of departures, de-icing times, and departure times such that `t` is a valid de-icing time
// for departure `i`, and `u` is a valid departure time for `i`
setof(DepDeIceSched) PossibleDepDeIceScheds = { <i, t, u> |
	i in Departures, t in PossibleDeIceTimes[i], u in PossibleDepartureTimes[i] };

// TODO: Figure out a suitable cost for holdover time
dexpr int holdover[<i, t, u> in PossibleDepDeIceScheds] = (isSchedAt[<i, u>] + isDeIceAt[<i, t>])
	* ((u - t - deps[i].deIceDur) - (deps[i].taxiOutDur + deps[i].lineUpDur));

// Set of separation-identical flights `j` for a flight `i`, such that `j`
// has an ideal take-off or landing time after that of `i`
setof(int) SeparationIdenticalFlightsAfter[i in Flights] = { j | j, k in Flights:
	i != j && j != k
	&& sep[i, k] == sep[j, k]
	&& sep[k, i] == sep[k, j] };

tuple FlightPairSched {
  	int i;
  	int t;
  	
  	int j;
  	int u;
};

setof(FlightPairSched) PossibleSeparationIdenticalPairScheds = { <i, t, j, u> |
	<i, t> in PossibleScheds,
	<j, u> in PossibleScheds:
	j in SeparationIdenticalFlightsAfter[i]
	&& flights[j].window.target >= flights[i].window.target };

minimize (sum (<i, t> in PossibleScheds) delay[<i, t>])
	+ (sum (i in Departures) drop[i])
	+ (sum (<i, t, u> in PossibleDepDeIceScheds) holdover[<i, t, u>]);

subject to {
  	// There must be exactly one time `t` for which a flight `i` is scheduled to depart,
  	// and only if it is not dropped
  	ScheduleDepOrDrop:
	  	forall (i in Departures)
	  	  	(sum (t in PossibleDepartureTimes[i]) isSchedAt[<i, t>]) + isDropped[i] == 1;

  	// There must be exactly one time `t` for which a flight `i` is scheduled to arrive,
  	// and it must be within its time window
  	ScheduleArr:
	  	forall (i in Arrivals)
	  	  	(sum (t in PossibleArrivalTimes[i]) isSchedAt[<i, t>]) == 1;

  	// There must be exactly one time `t` for which a departure `i` is scheduled to de-ice,
  	// and only if it is not dropped
  	ScheduleDeIceOrDrop:
		forall (i in Departures)
	  	  	(sum (t in PossibleDeIceTimes[i]) isDeIceAt[<i, t>]) + isDropped[i] == 1;

  	// Any two flights `i` and `j` scheduled to arrive or depart at times `t` and `u`
  	// respectively must maintain the minimum separation time between them
  	MaintainSeparation:
	  	forall (<i, t> in PossibleScheds, <j, u> in PossibleScheds: i != j)
	  		(u >= t + sep[i, j] || t >= u + sep[j, i])
			|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

  	// A departing flight `i` must de-ice at a time `t` such that it has enough time to
  	// get to the runway and depart at its scheduled departure time `u`
  	DeIceBeforeDep:
	  	forall (<i, t, u> in PossibleDepDeIceScheds)
	  		(u >= t + deps[i].deIceDur
				+ deps[i].taxiOutDur
				+ deps[i].lineUpDur)
			|| !(isSchedAt[<i, u>] == true && isDeIceAt[<i, t>] == true);

	// A departure `i` scheduled to de-ice at time `t` can only start de-icing
	// when the previous departure `j` scheduled to de-ice at time `u` has finished,
	// or the other way around if `u` is after `t`
	NoDeIceOverlap:
		forall (<i, t> in PossibleDeIceScheds, <j, u> in PossibleDeIceScheds: i != j)
			(u >= t + deps[i].deIceDur || t >= u + deps[j].deIceDur)
			|| !(isDeIceAt[<i, t>] == true && isDeIceAt[<j, u>] == true);
	
	// A departure `i`'s holdover time must not exceed the maximum allowed holdover time
	AcceptableHoldover:
		forall (<i, t, u> in PossibleDepDeIceScheds)
			(u - t - deps[i].deIceDur) - (deps[i].taxiOutDur + deps[i].lineUpDur) <= maxHoldoverDur
			|| !(isSchedAt[<i, u>] == true && isDeIceAt[<i, t>] == true);

	forall (<i, t, j, u> in PossibleSeparationIdenticalPairScheds)
		u >= t
	  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);
};
