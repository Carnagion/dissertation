tuple TimeWindow {
  	int before;
  	int target;
  	int after;
};

int maxHoldoverDur = ...;
assert ValidMaxHoldover:
	maxHoldoverDur >= 0;

int maxSlackDur = ...;
assert ValidMaxSlack:
	maxSlackDur >= 0;

string departure = "departure";
string arrival = "arrival";

tuple Flight {
 	// NOTE: Always must be one of "departure" or "arrival"
  	string kind;
  	
  	TimeWindow window;
  	
  	// NOTE: For departures only
  	int pushbackDur;
  	int taxiDeiceDur;
  	int deiceDur;
  	int taxiOutDur;
  	int lineupDur;
  	
  	// NOTE: For arrivals only
  	int taxiInDur;
};

int nbOfFlights = ...;
assert ValidFlightCount:
	nbOfFlights >= 0;

// Indexes of flights
range Flights = 1..nbOfFlights;

// Set of flights to be scheduled - both departures and arrivals
Flight flights[i in Flights] = ...;
assert ValidFlightData:
	forall (i in Flights)
	  	flights[i].kind in { departure, arrival }
	  	&& (flights[i].window.before >= 0
	  		&& flights[i].window.after >= 0)
	  	&& (flights[i].kind == departure
	  		=> (flights[i].pushbackDur >= 0
	  			&& flights[i].taxiDeiceDur >= 0
	  			&& flights[i].deiceDur >= 0
	  			&& flights[i].taxiOutDur >= 0
	  			&& flights[i].lineupDur >= 0)
				&& (flights[i].taxiOutDur + flights[i].lineupDur <= maxHoldoverDur))
		&& (flights[i].kind == arrival
			=> (flights[i].taxiInDur >= 0));

int earliest[i in Flights] = flights[i].window.target - flights[i].window.before;

int latest[i in Flights] = flights[i].window.target + flights[i].window.after;

tuple Dep {
  	TimeWindow ctot;
  	
  	int pushbackDur;
  	int taxiDeiceDur;
  	int deiceDur;
  	int taxiOutDur;
  	int lineupDur;
};

// Indexes of departures
setof(int) Departures = { i | i in Flights: flights[i].kind == departure };

// Set of departures to be scheduled
Dep deps[i in Departures] = <
	flights[i].window,
	flights[i].pushbackDur,
	flights[i].taxiDeiceDur,
	flights[i].deiceDur,
	flights[i].taxiOutDur,
	flights[i].lineupDur>;
	
tuple Arr {
  	TimeWindow window;
  	
  	int taxiInDur;
};

// Indexes of arrivals
setof(int) Arrivals = { i | i in Flights: flights[i].kind == arrival };

// Set of arrivals to be scheduled
Arr arrs[i in Arrivals] = <
	flights[i].window,
	flights[i].taxiInDur>;

// Separation matrix of flights for which `sep[i, j]` represents the separation
// requirement between flight `i` and flight `j` where `j` goes after `i`
int sep[i in Flights, j in Flights] = ...;
assert ValidSeparations:
	forall (i, j in Flights: i != j) sep[i, j] > 0;

// Set of times a flight `i` could possibly be scheduled at
setof(int) PossibleFlightTimes[i in Flights] = { t | t in earliest[i]..latest[i] };

tuple FlightSched {
  	int flight;
  	int time;
};

// Set of flights and their possible scheduled take-off or landing times
setof(FlightSched) PossibleFlightScheds = { <i, t> | i in Flights, t in PossibleFlightTimes[i] };

// Set of times a departure `i` could possibly be scheduled at
setof(int) PossibleDepartureTimes[i in Departures] = PossibleFlightTimes[i];

// Set of times a departure `i` could possibly de-ice at
setof(int) PossibleDeIceTimes[i in Departures] = { t | t in
	(earliest[i]
		- deps[i].lineupDur
		- deps[i].taxiOutDur
		- deps[i].deiceDur
		- maxSlackDur)
	..(latest[i]
		- deps[i].lineupDur
		- deps[i].taxiOutDur
		- deps[i].deiceDur) };

tuple DepSched {
  	int dep;
  	int deiceTime;
  	int takeoffTime;
};

// Set of departures and their possible de-icing times
setof(FlightSched) PossibleDeIceScheds = { <i, t> | i in Departures, t in PossibleDeIceTimes[i] };

// Set of departures, their possible de-icing times, and their possible take-off times
setof(DepSched) PossibleDepScheds = { <i, deice, takeoff> |
	i in Departures,
	deice in PossibleDeIceTimes[i],
	takeoff in PossibleDepartureTimes[i] };

// Set of times an arrival `i` could possibly be scheduled at
setof(int) PossibleArrivalTimes[i in Arrivals] = PossibleFlightTimes[i];

tuple ArrSched {
  	int arr;
  	int landingTime;
};

// Set of arrivals and their possible landing times
setof(ArrSched) PossibleArrScheds = { <i, t> | i in Arrivals, t in PossibleArrivalTimes[i] };

tuple FlightPair {
  	int before;
  	int after;
};

// Set of pairs of flights `i` and `j` for which `i` definitely takes off or lands
// before `j`, and for which the separation constraint is always satisfied
setof(FlightPair) DisjointSeparatedWindowFlightPairs = { <i, j> |
	i, j in Flights:
	i != j
	&& latest[i] < earliest[j]
	&& latest[i] + sep[i, j] <= earliest[j] };

// Set of pairs of flights `i` and `j` for which `i` definitely takes off or lands
// before `j`, but for which the separation constraint is not necessarily always satisfied
setof(FlightPair) DisjointWindowFlightPairs = { <i, j> |
	i, j in Flights:
	i != j
	&& latest[i] < earliest[j]
	&& latest[i] + sep[i, j] > earliest[j] };

// Set of pairs of flights `i` and `j` for which `i` may or may not take off or land
// before `j`
setof(FlightPair) OverlappingWindowFlightPairs = { <i, j> |
	i, j in Flights:
	i != j
	&& (earliest[i] in earliest[j]..latest[j]
		|| latest[i] in earliest[j]..latest[j]
		|| earliest[j] in earliest[i]..latest[i]
		|| latest[j] in earliest[i]..latest[i]) };

// Set of pairs of separation-identical flights `i` and `j` for which `i` may or may not
// take off or land before `j`
setof(FlightPair) SeparationIdenticalFlightPairs = { <i, j> |
	<i, j> in OverlappingWindowFlightPairs,
	k in Flights:
	i != j && j != k
	&& flights[j].window.target > flights[i].window.target
	&& sep[i, k] == sep[j, k]
	&& sep[k, i] == sep[k, j] };

// Whether a flight `i` is scheduled at time `t`
dvar boolean isSchedAt[<i, t> in PossibleFlightScheds];

// Whether a departure `i` starts de-icing at time `t`
dvar boolean isDeiceAt[<i, t> in PossibleDeIceScheds];

dexpr int deviation[<i, t> in PossibleFlightScheds] = isSchedAt[<i, t>]
	* ftoi(pow(t - earliest[i], 2));

dexpr int violation[<i, t> in PossibleFlightScheds] = (isSchedAt[<i, t>] == true
	&& t not in earliest[i]..latest[i]) * ftoi(pow(60, 2));

dexpr int slack[<i, deice, takeoff> in PossibleDepScheds] = (isSchedAt[<i, takeoff>] + isDeiceAt[<i, deice>])
	* ftoi(pow(takeoff
		- deps[i].lineupDur
		- deps[i].taxiOutDur
		- deps[i].deiceDur
		- deice, 2));

minimize
  	// Minimize delay from earliest time for arrivals
  	sum (<i, landing> in PossibleArrScheds)
  	  	(deviation[<i, landing>]
  	  	+ violation[<i, landing>])
  	// Minimize delay from earliest time and waiting time for departures
	+ sum (<i, deice, takeoff> in PossibleDepScheds)
		(deviation[<i, takeoff>]
		+ violation[<i, takeoff>]
		+ slack[<i, deice, takeoff>]);

subject to {
  	// Each flight `i` must be scheduled exactly once
  	ScheduleFlightOrDrop:
	  	forall (i in Flights)
	  	  	sum (t in PossibleFlightTimes[i]) isSchedAt[<i, t>] == 1;

	// Each departure `i` must have de-icing scheduled exactly once
  	ScheduleDeIceOrDrop:
		forall (i in Departures)
	  	  	sum (t in PossibleDeIceTimes[i]) isDeiceAt[<i, t>] == 1;

	// De-icing for a departure `i` must happen before its scheduled departure time, with
	// enough time for the plane to get from the de-icing station to the runway
  	DeIceBeforeDeparture:
	  	forall (<i, deice, takeoff> in PossibleDepScheds)
	  		takeoff >= deice
				+ deps[i].lineupDur
				+ deps[i].taxiOutDur
	  			+ deps[i].deiceDur // NOTE: Slack not counted here - it only influences possible de-icing times
			|| !(isSchedAt[<i, takeoff>] == true && isDeiceAt[<i, deice>] == true);

	// Each departure `j` cannot start de-icing until the previous departure `i` finishes de-icing
	NoDeIceOverlap:
		forall (<i, t> in PossibleDeIceScheds, <j, u> in PossibleDeIceScheds: i != j)
			u >= t + deps[i].deiceDur
			|| t >= u + deps[j].deiceDur
			|| !(isDeiceAt[<i, t>] == true && isDeiceAt[<j, u>] == true);

	// Each departure `i` must have a holdover time below the allowed maximum
	AcceptableHoldover:
		forall (<i, deice, takeoff> in PossibleDepScheds)
			takeoff - deice - deps[i].deiceDur <= maxHoldoverDur
			|| !(isSchedAt[<i, takeoff>] == true && isDeiceAt[<i, deice>] == true);

	// Any two flights `i` and `j` with non-overlapping time windows but without automatically satisfying the separation
	// requirements must maintain separation between them
	CompleteOrderInDisjointWindowFlights:
		forall (<i, j> in DisjointWindowFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
		  	u >= t + sep[i, j]
		  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

	// Any two flights `i` and `j` with overlapping time windows must maintain separation between them
	MaintainSeparationInOverlappingWindowFlights:
		forall (<i, j> in OverlappingWindowFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
		  	u >= t
		  		+ sep[i, j] * (u >= t + 1)
		  		- (latest[i] - earliest[j]) * (t >= u + 1)
		  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

	// Any two separation-identical flights `i` and `j` form a complete order that does not need to be reversed
	CompleteOrderInSeparationIdenticalFlights:
		forall (<i, j> in SeparationIdenticalFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
		  	u >= t + 1
		  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);
};

// TODO:
// - sum t_a, x_a <= sum t_b, x_b
// - iterate over consecutive pairs only
