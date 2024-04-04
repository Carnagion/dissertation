tuple TimeWindow {
  	int earliestTime;
  	int duration;
};

tuple Ctot {
  	int targetTime;
  	int allowEarly;
  	int allowLate;
};

tuple Arrival {
  	int baseTime;
  	TimeWindow window;
};

tuple Deice {
  	int taxiDuration;
  	int duration;
  	int hot;
}

tuple Departure {
  	int baseTime;
  	int tobt;
  	int pushbackDuration;
  	Deice deice;
  	int taxiOutDuration;
  	int lineupDuration;
  	Ctot ctot;
  	TimeWindow window;
};

string arrival = "arrival";
string departure = "departure";

tuple Flight {
  	// NOTE: Must be either "arrival" or "departure" only
  	string kind;
  	
  	// NOTE: For both arrivals and departures
  	int baseTime;
  	
  	// NOTE: For departures only
  	int tobt;
  	int pushbackDuration;
  	Deice deice;
  	int taxiOutDuration;
  	int lineupDuration;
  	Ctot ctot;
  	
  	// NOTE: For both arrivals and departures
  	TimeWindow window;
};

int flightCount = ...;
assert NonNegativeFlightCount:
	flightCount >= 0;

range Flights = 1..flightCount;

Flight flights[i in Flights] = ...;
assert ValidFlights:
	forall (i in Flights)
	  	flights[i].kind in { arrival, departure }
	  	&& flights[i].window.duration >= 0;

int sep[i in Flights, j in Flights] = ...;

int isArrival[i in Flights] = flights[i].kind == arrival;
int isDeparture[i in Flights] = flights[i].kind == departure;

setof(int) Arrivals = { i | i in Flights: isArrival[i] == true };
setof(int) Departures = { i | i in Flights: isDeparture[i] == true };

Arrival arrs[i in Arrivals] = <
	flights[i].baseTime,
	flights[i].window>;

Departure deps[i in Departures] = <
	flights[i].baseTime,
	flights[i].tobt,
	flights[i].pushbackDuration,
	flights[i].deice,
	flights[i].taxiOutDuration,
	flights[i].lineupDuration,
	flights[i].ctot,
	flights[i].window>;
assert ValidDepartures:
	forall (i in Departures)
	  	deps[i].ctot.allowEarly >= 0
	  	&& deps[i].ctot.allowLate >= 0
	  	&& deps[i].pushbackDuration >= 0
	  	&& deps[i].taxiOutDuration >= 0
	  	&& deps[i].lineupDuration >= 0;

int mustDeice[i in Departures] = deps[i].deice.duration > 0;

setof(int) DeicedDepartures = { i | i in Departures: mustDeice[i] == true };
assert ValidDeicedDepartures:
	forall (i in DeicedDepartures)
	  	deps[i].deice.duration > 0
	  	&& deps[i].deice.taxiDuration >= 0
	  	&& deps[i].deice.hot >= 0;

int maxRunwayHold = ...;
assert ValidMaxRunwayHold:
	maxRunwayHold >= 0;
	
int maxTime = ...;
assert ValidMaxTime:
	maxTime >= 0;

int hasWindow[i in Flights] = flights[i].window.duration > 0;

int earliestWindowTime[i in Flights] = hasWindow[i] == true
	? flights[i].window.earliestTime
	: -maxTime;
int latestWindowTime[i in Flights] = hasWindow[i] == true
	? flights[i].window.earliestTime + flights[i].window.duration
	: maxTime;

int hasCtot[i in Flights] = isDeparture[i] == true
	&& flights[i].ctot.allowEarly > 0
	&& flights[i].ctot.allowLate > 0;

int earliestCtotTime[i in Departures] = deps[i].ctot.targetTime - deps[i].ctot.allowEarly;
int latestCtotTime[i in Departures] = deps[i].ctot.targetTime + deps[i].ctot.allowLate;

int arrReleaseTime[i in Arrivals] = maxl(arrs[i].baseTime, earliestWindowTime[i]);
int arrDueTime[i in Arrivals] = latestWindowTime[i];

int depReleaseTime[i in Departures] = hasCtot[i] == true
	? maxl(deps[i].baseTime, earliestCtotTime[i], earliestWindowTime[i])
	: maxl(deps[i].baseTime, earliestWindowTime[i]);
int depDueTime[i in Departures] = latestWindowTime[i];
	
int releaseTime[i in Flights] = isArrival[i] == true ? arrReleaseTime[i]
	: isDeparture[i] == true ? depReleaseTime[i]
	: maxTime + 1; // NOTE: This case is never reachable
int dueTime[i in Flights] = isArrival[i] == true ? arrDueTime[i]
	: isDeparture[i] == true ? depDueTime[i]
	: maxTime + 1; // NOTE: This case is never reachable

tuple FlightTimePair {
  	int flight;
  	int time;
};

setof(int) FlightTimes[i in Flights] = asSet(releaseTime[i]..dueTime[i]);

setof(FlightTimePair) PossibleFlightSchedules = { <i, t> | i in Flights, t in FlightTimes[i] };

int earliestDeiceTime[i in DeicedDepartures] = releaseTime[i]
	- maxRunwayHold
	- deps[i].lineupDuration
	- deps[i].taxiOutDuration
	- deps[i].deice.duration;

int latestDeiceTime[i in DeicedDepartures] = dueTime[i]
	- deps[i].lineupDuration
	- deps[i].taxiOutDuration
	- deps[i].deice.duration;

setof(int) DeiceTimes[i in DeicedDepartures] = asSet(earliestDeiceTime[i]..latestDeiceTime[i]);

setof(FlightTimePair) PossibleDeiceSchedules = { <i, t> | i in DeicedDepartures, t in DeiceTimes[i] };

tuple FlightPair {
  	int first;
  	int second;
};

int haveOverlappingWindows[i in Flights, j in Flights] =
	earliestWindowTime[j] <= earliestWindowTime[i] <= latestWindowTime[j]
	|| earliestWindowTime[j] <= latestWindowTime[i] <= latestWindowTime[j]
	|| earliestWindowTime[i] <= earliestWindowTime[j] <= latestWindowTime[i]
	|| earliestWindowTime[i] <= latestWindowTime[j] <= latestWindowTime[i];

int areSeparationIdentical[i in Flights, j in Flights] = prod (k in Flights:
	i != k && j != k)
	(sep[i, k] == sep[j, k] && sep[k, i] == sep[k, j]) == true;

int areCompleteOrdered[i in Flights, j in Flights] =
	hasCtot[i] == false && hasCtot[j] == false
	&& releaseTime[i] <= releaseTime[j]
	&& flights[i].baseTime <= flights[j].baseTime
	&& latestWindowTime[i] <= latestWindowTime[j]
	&& (j > i ||
		!(releaseTime[i] == releaseTime[j]
		&& flights[i].baseTime == flights[j].baseTime
		&& latestWindowTime[i] == latestWindowTime[j]));

setof(FlightPair) DistinctFlightPairs = { <i, j> | i, j in Flights: i != j };

setof(FlightPair) DisjointSeparatedWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	latestWindowTime[i] < earliestWindowTime[j]
	&& latestWindowTime[i] + sep[i, j] <= earliestWindowTime[j] };

setof(FlightPair) DisjointWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	latestWindowTime[i] < earliestWindowTime[j]
	&& latestWindowTime[i] + sep[i, j] > earliestWindowTime[j] };

setof(FlightPair) OverlappingWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	haveOverlappingWindows[i, j] == true };

setof(FlightPair) SeparationIdenticalCompleteOrderedFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	areSeparationIdentical[i, j] == true
	&& areCompleteOrdered[i, j] == true };

dvar boolean isScheduledAt[<i, t> in PossibleFlightSchedules];

dexpr int isScheduled[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>];

dexpr int scheduledTime[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>] * t;

dvar boolean areScheduledInOrder[<i, j> in DistinctFlightPairs];

dvar boolean startsDeiceAt[<i, t> in PossibleDeiceSchedules];

dexpr int isDeiced[i in DeicedDepartures] = sum (t in DeiceTimes[i]) startsDeiceAt[<i, t>];

dexpr int deiceTime[i in DeicedDepartures] = sum (t in DeiceTimes[i]) startsDeiceAt[<i, t>] * t;

dexpr int runwayHoldDuration[i in DeicedDepartures] = scheduledTime[i]
	- deps[i].lineupDuration
	- deps[i].taxiOutDuration
	- deps[i].deice.duration
	- deiceTime[i];

dexpr int delayCost[i in Flights] = sum (t in FlightTimes[i])
	isScheduledAt[<i, t>] * ftoi(pow(t - flights[i].baseTime, 2));

dexpr int ctotViolationCost[i in Departures] = hasCtot[i] * sum (t in FlightTimes[i])
	(t >= latestCtotTime[i] + 1)
	* ftoi(pow(t - latestCtotTime[i], 2));
	
minimize sum (i in Arrivals) delayCost[i]
  	+ sum (i in Departures) (delayCost[i] + ctotViolationCost[i]);

subject to {
  	ScheduleFlightsOnce:
	  	forall (i in Flights)
	  	  	isScheduled[i] == true;

	NoScheduleOverlap:
		forall (<i, j> in DistinctFlightPairs)
		  	areScheduledInOrder[<i, j>] + areScheduledInOrder[<j, i>] == true;
  	
  	DeiceFlightsOnce:
  		forall (i in DeicedDepartures)
  		  	isDeiced[i] == true;

	NoDeiceOverlap:
		forall (i, j in DeicedDepartures: i != j)
		  	deiceTime[j] >= deiceTime[i] + deps[i].deice.duration
		  	|| deiceTime[i] >= deiceTime[j] + deps[j].deice.duration;

  	AllowedHoldover:
  		forall (i in DeicedDepartures)
  		  	scheduledTime[i] >= deiceTime[i]
  		  		+ deps[i].deice.duration
  		  		+ deps[i].taxiOutDuration
  		  		+ deps[i].lineupDuration
  		  	&& scheduledTime[i] - deiceTime[i] - deps[i].deice.duration <= deps[i].deice.hot
  		  	&& runwayHoldDuration[i] <= maxRunwayHold;

	MinimumSeparation: {
	  	InDisjointSeparatedWindowFlights:
	  		forall (<i, j> in DisjointSeparatedWindowFlightPairs)
	  		  	areScheduledInOrder[<i, j>] == true;

	  	InDisjointWindowFlights:
	  		forall (<i, j> in DisjointWindowFlightPairs)
	  	  		scheduledTime[j] >= scheduledTime[i] + sep[i, j]
	  	  		&& areScheduledInOrder[<i, j>] == true;

  		InOverlappingWindowFlights:
  			forall (<i, j> in OverlappingWindowFlightPairs)
	  		  	scheduledTime[j] >= scheduledTime[i]
			  		+ sep[i, j]
			  			* areScheduledInOrder[<i, j>]
			  		- (latestWindowTime[i] - earliestWindowTime[j])
			  			* areScheduledInOrder[<j, i>];

	CompleteOrders:
		forall (<i, j> in SeparationIdenticalCompleteOrderedFlightPairs)
		  	scheduledTime[j] >= scheduledTime[i] + sep[i, j]
  			&& areScheduledInOrder[<i, j>] == true;
  	};
}

//execute SaveSolution {
//  	var solution = new Array(flightCount);
//  	for (var i in Flights) {
//  	  	var flight = flights[i];
//
//  	  	var sched = new Object();
//  	  	sched.flightIdx = i - 1;
//  	  	if (flight.kind == arrival) {
//  	  	  	sched.landing = scheduledTime[i];
//  	  	} else if (flight.kind == departure) {
//  	  	  	sched.takeoff = scheduledTime[i];
//  	  	  	if (mustDeice[i]) {
//  	  	  		sched.deice = deiceTime[i];
//  	  	  	}
//  	  	}
//
//  	  	solution[i] = sched;
//  	}
//
//  	function compareSchedules(sched, other) {
//  	  	var schedTime = scheduledTime[sched.flightIdx + 1];
//  	  	var otherTime = scheduledTime[other.flightIdx + 1];
//  	  	if (schedTime < otherTime) {
//  	  	  	return -1;
//  	  	} else if (schedTime > otherTime) {
//  	  	  	return 1;
//  	  	} else {
//  	  	  	return 0;
//  	  	}
//  	}
//
//  	solution.sort(compareSchedules);
//  	
//  	// TODO: Change the start time when testing a different instance
//  	var startTime = new Object();
//  	startTime.hour = 22;
//  	startTime.minute = 25;
//  	startTime.second = 30;
//  	
//  	function toHms(secondOffset) {
//  	  	var startSecond = (startTime.hour * 3600) + (startTime.minute * 60) + startTime.second;
//  	  	
//  	  	var schedSecond = startSecond + secondOffset;
//  	  	
//  	  	var hour = Math.floor(schedSecond / 3600);
//  	  	var minute = Math.floor(schedSecond % 3600 / 60);
//  	  	var second = schedSecond % 3600 % 60;
//  	  	
//  	  	var datetime = new Date(2024, 1, 24, hour, minute, second, 0).toUTCString().split(" ");
//  	  	var date = datetime[0].split("/");
//  	  	var day = date[1];
//  	  	var month = date[0];
//  	  	var year = date[2];
//  	  	var time = datetime[1];
//  	  	
//  	  	return "\"" + year + "-" + month + "-" + day + "T" + time + "\"";
//  	}
//
//	// TODO: Change output path when testing a different instance
//  	var file = new IloOplOutputFile("../../solutions/heathrow/10.toml");
//  	for (var i in Flights) {
//  	  	var sched = solution[i - 1];
//  	  	var flight = flights[sched.flightIdx + 1];
//  	  	
//  	  	file.writeln("[[schedules]]")
//  	  	file.writeln("flight-idx = ", sched.flightIdx);
//  	  	if (flight.kind == arrival) {
//  	  	  	file.writeln("kind = \"arrival\"");
//  	  	  	file.writeln("landing = ", toHms(sched.landing));
//  	  	} else if (flight.kind == departure) {
//  	  	  	file.writeln("kind = \"departure\"");
//  	  	  	if (mustDeice[sched.flightIdx + 1]) {
//  	  	  		file.writeln("deice = ", toHms(sched.deice));
//  	  	  	}
//  	  	  	file.writeln("takeoff = ", toHms(sched.takeoff));
//  	  	}
//  	  	file.writeln();
//  	}
//};
