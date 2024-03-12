tuple TimeWindow {
  	int earliestTime;
  	int latestTime;
};

tuple Ctot {
  	int targetTime;
  	int allowBefore;
  	int allowAfter;
};

tuple Arrival {
  	int baseTime;
  	TimeWindow window;
};

tuple Departure {
  	int baseTime;
  	TimeWindow window;
  	Ctot ctot;
  	int pushbackDur;
  	int taxiDeiceDur;
  	int deiceDur;
  	int taxiOutDur;
  	int lineupDur;
};

string arrival = "arrival";
string departure = "departure";

tuple Flight {
  	// NOTE: Must be either "arrival" or "departure" only
  	string kind;
  	
  	// NOTE: For both arrivals and departures
  	int baseTime;
  	TimeWindow window;
  	
  	// NOTE: For departures only
  	Ctot ctot;
  	int pushbackDur;
  	int taxiDeiceDur;
  	int deiceDur;
  	int taxiOutDur;
  	int lineupDur;
};

int flightCount = ...;
assert NonNegativeFlightCount:
	flightCount >= 0;

range Flights = 1..flightCount;

Flight flights[i in Flights] = ...;
assert ValidFlights:
	forall (i in Flights)
	  	flights[i].kind in { arrival, departure }
	  	&& flights[i].window.earliestTime <= flights[i].window.latestTime
	  	&& flights[i].baseTime <= flights[i].window.latestTime;

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
	flights[i].window,
	flights[i].ctot,
	flights[i].pushbackDur,
	flights[i].taxiDeiceDur,
	flights[i].deiceDur,
	flights[i].taxiOutDur,
	flights[i].lineupDur>;
assert ValidDepartures:
	forall (i in Departures)
	  	deps[i].ctot.targetTime <= deps[i].window.latestTime
	  	&& deps[i].ctot.allowBefore >= 0
	  	&& deps[i].ctot.allowAfter >= 0
	  	&& deps[i].pushbackDur >= 0
	  	&& deps[i].taxiDeiceDur >= 0
	  	&& deps[i].deiceDur >= 0
	  	&& deps[i].taxiOutDur >= 0
	  	&& deps[i].lineupDur >= 0;

int maxAllowedHoldover = ...;
assert ValidMaxAllowedHoldover:
	maxAllowedHoldover >= 0;

int maxAllowedSlack = ...;
assert ValidMaxAllowedSlack:
	maxAllowedSlack >= 0;

// TODO: Work on everything below this point

int hasCtot[i in Departures] = deps[i].ctot.allowBefore > 0 && deps[i].ctot.allowAfter > 0;

int earliestCtotTime[i in Departures] = deps[i].ctot.targetTime - deps[i].ctot.allowBefore;
int latestCtotTime[i in Departures] = deps[i].ctot.targetTime + deps[i].ctot.allowAfter;

int arrReleaseTime[i in Arrivals] = maxl(arrs[i].window.earliestTime, arrs[i].baseTime);
int arrDueTime[i in Arrivals] = arrs[i].window.latestTime;

int depReleaseTime[i in Departures] = hasCtot[i] == true ? maxl(
	earliestCtotTime[i],
	deps[i].window.earliestTime,
	deps[i].baseTime) : maxl(deps[i].window.earliestTime, deps[i].baseTime);
int depDueTime[i in Departures] = deps[i].window.latestTime;

int releaseTime[i in Flights] = isArrival[i] == true ? arrReleaseTime[i]
	: isDeparture[i] == true ? depReleaseTime[i]
	: 0;
int dueTime[i in Flights] = isArrival[i] == true ? arrDueTime[i]
	: isDeparture[i] == true ? depDueTime[i]
	: 0;

tuple FlightTimePair {
  	int flight;
  	int time;
};

setof(int) FlightTimes[i in Flights] = asSet(releaseTime[i]..dueTime[i]);

setof(FlightTimePair) PossibleFlightSchedules = { <i, t> | i in Flights, t in FlightTimes[i] };

int earliestDeiceTime[i in Departures] = releaseTime[i]
	- maxAllowedSlack
	- deps[i].lineupDur
	- deps[i].taxiOutDur
	- deps[i].deiceDur;

int latestDeiceTime[i in Departures] = dueTime[i]
	- deps[i].lineupDur
	- deps[i].taxiOutDur
	- deps[i].deiceDur;

setof(int) DeiceTimes[i in Departures] = asSet(earliestDeiceTime[i]..latestDeiceTime[i]);

setof(FlightTimePair) PossibleDeiceSchedules = { <i, t> | i in Departures, t in DeiceTimes[i] };

tuple FlightPair {
  	int first;
  	int second;
};

int haveOverlappingWindows[i in Flights, j in Flights] =
	flights[j].window.earliestTime <= flights[i].window.earliestTime <= flights[j].window.latestTime
	|| flights[j].window.earliestTime <= flights[i].window.latestTime <= flights[j].window.latestTime
	|| flights[i].window.earliestTime <= flights[j].window.earliestTime <= flights[i].window.latestTime
	|| flights[i].window.earliestTime <= flights[j].window.latestTime <= flights[i].window.latestTime;

int areSeparationIdentical[i in Flights, j in Flights] = prod (k in Flights:
	i != k && j != k)
	(sep[i, k] == sep[j, k] && sep[k, i] == sep[k, j]) == true;

int areCompleteOrdered[i in Arrivals, j in Arrivals] =
	releaseTime[i] <= releaseTime[j]
	&& arrs[i].baseTime <= arrs[j].baseTime
	&& arrs[i].window.latestTime <= arrs[j].window.latestTime
	&& (j > i ||
		!(releaseTime[i] == releaseTime[j]
		&& arrs[i].baseTime == arrs[j].baseTime
		&& arrs[i].window.latestTime == arrs[j].window.latestTime));

setof(FlightPair) DistinctFlightPairs = { <i, j> | i, j in Flights: i != j };

setof(FlightPair) DisjointSeparatedWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	flights[i].window.latestTime < flights[j].window.earliestTime
	&& flights[i].window.latestTime + sep[i, j] <= flights[j].window.earliestTime };

setof(FlightPair) DisjointWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	flights[i].window.latestTime < flights[j].window.earliestTime
	&& flights[i].window.latestTime + sep[i, j] > flights[j].window.earliestTime };

setof(FlightPair) OverlappingWindowFlightPairs = { <i, j> | <i, j> in DistinctFlightPairs:
	haveOverlappingWindows[i, j] == true };

setof(FlightPair) SeparationIdenticalCompleteOrderedFlightPairs = { <i, j> | i, j in Arrivals:
	i != j
	&& areSeparationIdentical[i, j] == true
	&& areCompleteOrdered[i, j] == true };

dvar boolean isScheduledAt[<i, t> in PossibleFlightSchedules];

dexpr int isScheduled[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>];

dexpr int scheduledTime[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>] * t;

dvar boolean areScheduledInOrder[<i, j> in DistinctFlightPairs];

dvar boolean startsDeiceAt[<i, t> in PossibleDeiceSchedules];

dexpr int isDeiced[i in Departures] = sum (t in DeiceTimes[i]) startsDeiceAt[<i, t>];

dexpr int deiceTime[i in Departures] = sum (t in DeiceTimes[i]) startsDeiceAt[<i, t>] * t;

dexpr int deiceSlack[i in Departures] = scheduledTime[i]
	- deps[i].lineupDur
	- deps[i].taxiOutDur
	- deps[i].deiceDur
	- deiceTime[i];

dexpr int delayCost[i in Flights] = sum (t in FlightTimes[i])
	isScheduledAt[<i, t>] * ftoi(pow(t - flights[i].baseTime, 2));

dexpr int ctotViolationCost[i in Departures] = hasCtot[i]
	* (scheduledTime[i] >= latestCtotTime[i] + 1)
	* ftoi(pow(60, 2));

dexpr int slackCost[i in Departures] = sum (takeoff in FlightTimes[i], deice in DeiceTimes[i])
	(isScheduledAt[<i, takeoff>] == true && startsDeiceAt[<i, deice>] == true)
	* ftoi(pow(takeoff - deps[i].lineupDur - deps[i].taxiOutDur - deps[i].deiceDur - deice, 2));

minimize sum (i in Arrivals) delayCost[i]
  	+ sum (i in Departures) (delayCost[i] + ctotViolationCost[i] + slackCost[i]);

subject to {
  	ScheduleFlightsOnce:
	  	forall (i in Flights)
	  	  	isScheduled[i] == true;

	NoScheduleOverlap:
		forall (<i, j> in DistinctFlightPairs)
		  	areScheduledInOrder[<i, j>] + areScheduledInOrder[<j, i>] == true;
  	
  	DeiceFlightsOnce:
  		forall (i in Departures)
  		  	isDeiced[i] == true;

	NoDeiceOverlap:
		forall (i, j in Departures: i != j)
		  	deiceTime[j] >= deiceTime[i] + deps[i].deiceDur
		  	|| deiceTime[i] >= deiceTime[j] + deps[j].deiceDur;

  	AllowedHoldover:
  		forall (i in Departures)
  		  	scheduledTime[i] >= deiceTime[i] + deps[i].deiceDur + deps[i].taxiOutDur + deps[i].lineupDur
  		  	&& scheduledTime[i] - deiceTime[i] - deps[i].deiceDur <= maxAllowedHoldover
  		  	&& deiceSlack[i] <= maxAllowedSlack;

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
			  		- (flights[i].window.latestTime - flights[j].window.earliestTime)
			  			* areScheduledInOrder[<j, i>];
  	};

	CompleteOrders:
		forall (<i, j> in SeparationIdenticalCompleteOrderedFlightPairs)
		  	scheduledTime[j] >= scheduledTime[i] + sep[i, j]
  			&& areScheduledInOrder[<i, j>] == true;
};

execute SaveSolution {
  	var solution = new Array(flightCount);
  	for (var i in Flights) {
  	  	var flight = flights[i];

  	  	var sched = new Object();
  	  	sched.flightIdx = i - 1;
  	  	if (flight.kind == arrival) {
  	  	  	sched.landing = scheduledTime[i];
  	  	} else if (flight.kind == departure) {
  	  	  	sched.takeoff = scheduledTime[i];
  	  	  	sched.deice = deiceTime[i];
  	  	}

  	  	solution[i] = sched;
  	}

  	function compareSchedules(sched, other) {
  	  	var schedTime = scheduledTime[sched.flightIdx + 1];
  	  	var otherTime = scheduledTime[other.flightIdx + 1];
  	  	if (schedTime < otherTime) {
  	  	  	return -1;
  	  	} else if (schedTime > otherTime) {
  	  	  	return 1;
  	  	} else {
  	  	  	return 0;
  	  	}
  	}

  	solution.sort(compareSchedules);
  	
  	var startTime = new Object();
  	startTime.hour = 14;
  	startTime.minute = 55;
  	
  	function toHourMinute(minuteOffset) {
  	  	var startMinute = (startTime.hour * 60) + startTime.minute;
  	  	var schedMinute = startMinute + minuteOffset;
  	  	
  	  	var hour = Math.floor(schedMinute / 60);
  	  	var minute = schedMinute % 60;
  	  	
  	  	var date = new Date(0, 0, 1, hour, minute, 0, 0);
  	  	var time = date.toUTCString().split(" ")[1];
  	  	
  	  	return "\"" + time + "\"";
  	}

  	var file = new IloOplOutputFile("../solutions/furini/test.toml");
  	for (var i in Flights) {
  	  	var sched = solution[i - 1];
  	  	var flight = flights[sched.flightIdx + 1];
  	  	
  	  	file.writeln("[[schedules]]")
  	  	file.writeln("flight-idx = ", sched.flightIdx);
  	  	if (flight.kind == arrival) {
  	  	  	file.writeln("kind = \"arrival\"");
  	  	  	file.writeln("landing = ", toHourMinute(sched.landing));
  	  	} else if (flight.kind == departure) {
  	  	  	file.writeln("kind = \"departure\"");
  	  	  	file.writeln("deice = ", toHourMinute(sched.deice));
  	  	  	file.writeln("takeoff = ", toHourMinute(sched.takeoff));
  	  	}
  	  	file.writeln();
  	}
};
