tuple TimeWindow {
  	int earliest;
  	int target;
  	int latest;
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
  	int pushBackDur;
  	int taxiDeIceDur;
  	int deIceDur;
  	int taxiOutDur;
  	int lineUpDur;
  	
  	// NOTE: For arrivals only
  	int taxiInDur;
};

int nbOfFlights = ...;
assert ValidFlightCount:
	nbOfFlights >= 0;

range Flights = 1..nbOfFlights;

Flight flights[i in Flights] = ...;
assert ValidFlightData:
	forall (i in Flights)
	  	flights[i].kind in { departure, arrival }
	  	&& (flights[i].window.earliest
	  		<= flights[i].window.target
  			<= flights[i].window.latest)
	  	&& (flights[i].kind == departure
	  		=> (flights[i].pushBackDur >= 0
	  			&& flights[i].taxiDeIceDur >= 0
	  			&& flights[i].deIceDur >= 0
	  			&& flights[i].taxiOutDur >= 0
	  			&& flights[i].lineUpDur >= 0)
				&& (flights[i].taxiOutDur + flights[i].lineUpDur <= maxHoldoverDur))
		&& (flights[i].kind == arrival
			=> (flights[i].taxiInDur >= 0));

tuple Dep {
  	TimeWindow ctot;
  	
  	int pushBackDur;
  	int taxiDeIceDur;
  	int deIceDur;
  	int taxiOutDur;
  	int lineUpDur;
};

setof(int) Departures = { i | i in Flights: flights[i].kind == departure };

Dep deps[i in Departures] = <
	flights[i].window,
	flights[i].pushBackDur,
	flights[i].taxiDeIceDur,
	flights[i].deIceDur,
	flights[i].taxiOutDur,
	flights[i].lineUpDur>;
	
tuple Arr {
  	TimeWindow window;
  	
  	int taxiInDur;
};

setof(int) Arrivals = { i | i in Flights: flights[i].kind == arrival };

Arr arrs[i in Arrivals] = <
	flights[i].window,
	flights[i].taxiInDur>;

int sep[i in Flights, j in Flights] = ...;
assert ValidSeparations:
	forall (i, j in Flights: i != j) sep[i, j] > 0;

setof(int) PossibleFlightTimes[i in Flights] = { t | t in flights[i].window.earliest..flights[i].window.latest };

tuple FlightSched {
  	int flight;
  	int time;
};

setof(FlightSched) PossibleFlightScheds = { <i, t> | i in Flights, t in PossibleFlightTimes[i] };

setof(int) PossibleDepartureTimes[i in Departures] = PossibleFlightTimes[i];

setof(int) PossibleDeIceTimes[i in Departures] = { t | t in
	(deps[i].ctot.earliest
		- (deps[i].lineUpDur
			+ deps[i].taxiOutDur
			+ deps[i].deIceDur
			+ maxSlackDur))
	..(deps[i].ctot.latest
		- (deps[i].lineUpDur
			+ deps[i].taxiOutDur
			+ deps[i].deIceDur)) };

tuple DepSched {
  	int dep;
  	int deIceTime;
  	int takeOffTime;
};

setof(FlightSched) PossibleDeIceScheds = { <i, t> | i in Departures, t in PossibleDeIceTimes[i] };

setof(DepSched) PossibleDepScheds = { <i, t, u> | i in Departures,
	t in PossibleDeIceTimes[i],
	u in PossibleDepartureTimes[i] };

setof(int) PossibleArrivalTimes[i in Arrivals] = PossibleFlightTimes[i];

tuple ArrSched {
  	int arr;
  	int landingTime;
};

setof(ArrSched) PossibleArrScheds = { <i, t> | i in Arrivals, t in PossibleArrivalTimes[i] };

tuple FlightPair {
  	int before;
  	int after;
};

// Set of pairs of flights `i` and `j` for which `i` definitely takes off or lands
// before `j`, and for which the separation constraint is always satisfied
setof(FlightPair) DisjointSeparatedWindowFlightPairs = { <i, j> |
	<i, t> in PossibleFlightScheds,
	<j, u> in PossibleFlightScheds:
	i != j
	&& flights[i].window.latest < flights[j].window.earliest
	&& flights[i].window.latest + sep[i, j] <= flights[j].window.earliest };

// Set of pairs of flights `i` and `j` for which `i` definitely takes off or lands
// before `j`, but for which the separation constraint is not necessarily always satisfied
setof(FlightPair) DisjointWindowFlightPairs = { <i, j> |
	<i, t> in PossibleFlightScheds,
	<j, u> in PossibleFlightScheds:
	i != j
	&& flights[i].window.latest < flights[j].window.earliest
	&& flights[i].window.latest + sep[i, j] > flights[j].window.earliest };

// Set of pairs of flights `i` and `j` for which `i` may or may not take off or land
// before `j`
setof(FlightPair) OverlappingWindowFlightPairs = { <i, j> |
	<i, t> in PossibleFlightScheds,
	<j, u> in PossibleFlightScheds:
	i != j
	&& (flights[i].window.earliest in flights[j].window.earliest..flights[j].window.latest
		|| flights[i].window.latest in flights[j].window.earliest..flights[j].window.latest
		|| flights[j].window.earliest in flights[i].window.earliest..flights[i].window.latest
		|| flights[j].window.latest in flights[i].window.earliest..flights[i].window.latest) };

// Set of pairs of separation-identical flights `i` and `j` for which `i` may or may not
// take off or land before `j`
setof(FlightPair) SeparationIdenticalFlightPairs = { <i, j> |
	<i, j> in OverlappingWindowFlightPairs,
	k in Flights:
	i != j && j != k
	&& flights[j].window.target > flights[i].window.target
	&& sep[i, k] == sep[j, k]
	&& sep[k, i] == sep[k, j] };

dvar boolean isSchedAt[<i, t> in PossibleFlightScheds];

dvar boolean isDropped[i in Departures];

dvar boolean isDeIceAt[<i, t> in PossibleDeIceScheds];

dexpr int delay[<i, t> in PossibleFlightScheds] = isSchedAt[<i, t>] * (
	(t in flights[i].window.earliest..flights[i].window.target - 1)
		* ftoi(pow(flights[i].window.target - t, 2))
	+ (t in flights[i].window.target + 1..flights[i].window.latest)
		* ftoi(pow(t - flights[i].window.target, 2))
);

dexpr int drop[i in Departures] = isDropped[i] * 1000;

dexpr int holdover[<i, t, u> in PossibleDepScheds] = (isSchedAt[<i, u>] + isDeIceAt[<i, t>])
	* (u - (t + deps[i].deIceDur));

minimize (sum (<i, t> in PossibleFlightScheds) delay[<i, t>])
	+ (sum (i in Departures) drop[i])
	+ (sum (<i, t, u> in PossibleDepScheds) holdover[<i, t, u>]);

subject to {
  	ScheduleDepOrDrop:
	  	forall (i in Departures)
	  	  	(sum (t in PossibleDepartureTimes[i]) isSchedAt[<i, t>]) + isDropped[i] == 1;

  	ScheduleArr:
	  	forall (i in Arrivals)
	  	  	(sum (t in PossibleArrivalTimes[i]) isSchedAt[<i, t>]) == 1;

  	ScheduleDeIceOrDrop:
		forall (i in Departures)
	  	  	(sum (t in PossibleDeIceTimes[i]) isDeIceAt[<i, t>]) + isDropped[i] == 1;

  	DeIceBeforeDep:
	  	forall (<i, t, u> in PossibleDepScheds)
	  		(u >= t
				+ deps[i].lineUpDur
				+ deps[i].taxiOutDur
	  			+ deps[i].deIceDur
				+ maxSlackDur)
			|| !(isSchedAt[<i, u>] == true && isDeIceAt[<i, t>] == true);

	NoDeIceOverlap:
		forall (<i, t> in PossibleDeIceScheds, <j, u> in PossibleDeIceScheds: i != j)
			(u >= t + deps[i].deIceDur
			|| t >= u + deps[j].deIceDur)
			|| !(isDeIceAt[<i, t>] == true && isDeIceAt[<j, u>] == true);

	AcceptableHoldover:
		forall (<i, t, u> in PossibleDepScheds)
			(u - (t + deps[i].deIceDur)) <= maxHoldoverDur
			|| !(isSchedAt[<i, u>] == true && isDeIceAt[<i, t>] == true);

	forall (<i, j> in DisjointSeparatedWindowFlightPairs union DisjointWindowFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
	  	u >= t
	  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

	forall (<i, j> in DisjointWindowFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
	  	u >= t + sep[i, j]
	  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

	forall (<i, j> in OverlappingWindowFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
	  	u >= t
	  		+ sep[i, j] * (u >= t + 1)
	  		- (flights[i].window.latest - flights[j].window.earliest) * (t >= u + 1)
	  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);

	CompleteOrderInSeparationIdenticalFlights:
		forall (<i, j> in SeparationIdenticalFlightPairs, t in PossibleFlightTimes[i], u in PossibleFlightTimes[j])
		  	u >= t + 1
		  	|| !(isSchedAt[<i, t>] == true && isSchedAt[<j, u>] == true);
};
