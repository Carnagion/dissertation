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
  	
  	// NOTE: For arrivals only
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

range timeWindow[i in Flights] = flights[i].window.earliestTime..flights[i].window.latestTime;

int earliestCtotTime[i in Departures] = deps[i].ctot.targetTime - deps[i].ctot.allowBefore;
int latestCtotTime[i in Departures] = deps[i].ctot.targetTime + deps[i].ctot.allowAfter;

int earliestArrTime[i in Arrivals] = maxl(arrs[i].window.earliestTime, arrs[i].baseTime);
int latestArrTime[i in Arrivals] = arrs[i].window.latestTime;

int earliestDepTime[i in Departures] = maxl(
	earliestCtotTime[i],
	deps[i].window.earliestTime,
	deps[i].baseTime);
int latestDepTime[i in Departures] = deps[i].window.latestTime; // NOTE: Going over the CTOT end time
																//       is allowed, albeit heavily
																//       penalised

int earliestTime[i in Flights] = isArrival[i] == true ? earliestArrTime[i]
	: isDeparture[i] == true ? earliestDepTime[i]
	: 0;
int latestTime[i in Flights] = isArrival[i] == true ? latestArrTime[i]
	: isDeparture[i] == true ? latestDepTime[i]
	: 0;

int releaseTime[i in Flights] = earliestTime[i];

int minHoldoverDur[i in Departures] = deps[i].taxiOutDur + deps[i].lineupDur;
int maxHoldoverDur[i in Departures] = minl(maxAllowedHoldover,
	deps[i].taxiOutDur
	+ deps[i].lineupDur
	+ maxAllowedSlack);

int minTime = minl(
	min (i in Arrivals) minl(arrs[i].window.earliestTime, arrs[i].baseTime),
	min (i in Departures) minl(earliestCtotTime[i], deps[i].window.earliestTime, deps[i].baseTime)
);
int maxTime = maxl(
	max (i in Arrivals) arrs[i].window.latestTime,
	max (i in Departures) maxl(latestCtotTime[i], deps[i].window.latestTime)
);

range Time = minTime..maxTime;

setof(int) FlightTimes[i in Flights] = { t | t in earliestTime[i]..latestTime[i] };
setof(int) ArrivalTimes[i in Arrivals] = { t | t in earliestArrTime[i]..latestArrTime[i] };
setof(int) DepartureTimes[i in Departures] = { t | t in earliestDepTime[i]..latestDepTime[i] };

//range DeiceTimes[i in Departures] = earliestDepTime[i] - maxHoldoverDur[i] - deps[i].deiceDur
//	..latestDepTime[i] - minHoldoverDur[i] - deps[i].deiceDur;

tuple FlightTimePair {
  	int flight;
  	int time;
};

setof(FlightTimePair) PossibleFlightSchedules = { <i, t> | i in Flights, t in FlightTimes[i] };
setof(FlightTimePair) PossibleLandingSchedules = { <i, t> | i in Arrivals, t in ArrivalTimes[i] };
setof(FlightTimePair) PossibleTakeoffSchedules = { <i, t> | i in Departures, t in DepartureTimes[i] };
//setof(FlightTimePair) PossibleDeiceSchedules = { <i, t> | i in Departures, t in DeiceTimes[i] };

tuple FlightPair {
  	int first;
  	int second;
};

// Any two flights `i` and `j` with disjoint time windows are consecutive if there is no other
// flight with a time window disjoint to `i` and `j` between them.
// TODO: Check that it works
int haveConsecutiveDisjointWindows[i in Flights, j in Flights] = card({ k | k in Flights:
	flights[i].window.latestTime < flights[k].window.earliestTime
	&& flights[k].window.latestTime < flights[j].window.earliestTime }) == 0;

setof(FlightPair) DisjointSeparatedWindowConsecutivePairs = { <i, j> | i, j in Flights:
	i != j
	&& flights[i].window.latestTime < flights[j].window.earliestTime
	&& flights[i].window.latestTime + sep[i, j] <= flights[j].window.earliestTime
	&& haveConsecutiveDisjointWindows[i, j] == true };

setof(FlightPair) DisjointWindowConsecutivePairs = { <i, j> | i, j in Flights:
	i != j
	&& flights[i].window.latestTime < flights[j].window.earliestTime
	&& flights[i].window.latestTime + sep[i, j] > flights[j].window.earliestTime
	&& haveConsecutiveDisjointWindows[i, j] == true };

int haveOverlappingWindows[i in Flights, j in Flights] = earliestTime[i] in earliestTime[j]..latestTime[j]
		|| latestTime[i] in earliestTime[j]..latestTime[j]
		|| earliestTime[j] in earliestTime[i]..latestTime[i]
		|| latestTime[j] in earliestTime[i]..latestTime[i];

setof(FlightPair) OverlappingWindowFlightPairs = { <i, j> | i, j in Flights:
	i != j
	&& haveOverlappingWindows[i, j] == true };

// TODO: Check that it works
int areSeparationIdentical[i in Flights, j in Flights] = prod (k in Flights:
	i != k && j != k)
	(sep[i, k] == sep[j, k] && sep[k, i] == sep[k, j]) == true;

// TODO: Check that it works and find a way to compress this to only consecutive pairs
setof(FlightPair) SeparationIdenticalFlightPairs = { <i, j> | i, j in Flights:
	j > i
	&& areSeparationIdentical[i, j] == true
	&& releaseTime[j] >= releaseTime[i] };

dvar boolean isScheduledAt[<i, t> in PossibleFlightSchedules];

dexpr int isScheduled[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>];

dexpr int scheduledTime[i in Flights] = sum (t in FlightTimes[i]) isScheduledAt[<i, t>] * t;

dvar int slackDur[i in Departures] in 0..maxAllowedSlack;

dexpr int deiceTime[i in Departures] = scheduledTime[i]
	- slackDur[i]
	- deps[i].lineupDur
	- deps[i].taxiOutDur
	- deps[i].deiceDur;

dexpr int delay[i in Flights] = sum (t in FlightTimes[i])
	isScheduledAt[<i, t>] * ftoi(pow(t - flights[i].baseTime, 2));

dexpr int ctotViolation[i in Departures] = (scheduledTime[i] >= latestCtotTime[i] + 1) * ftoi(pow(60, 2));

// TODO: Find a way to square this
dexpr int deiceSlack[i in Departures] = slackDur[i];

minimize sum (i in Arrivals) delay[i]
  	+ sum (i in Departures) (delay[i] + ctotViolation[i] + deiceSlack[i]);

subject to {
  	ScheduleFlightsOnce:
	  	forall (i in Flights)
	  	  	isScheduled[i] == true;

	NoDeiceOverlap:
		forall (i, j in Departures: i != j)
		  	deiceTime[j] >= deiceTime[i] + deps[i].deiceDur
		  	|| deiceTime[i] >= deiceTime[j] + deps[j].deiceDur;

  	AllowedHoldover:
  		forall (i in Departures)
  		  	maxAllowedHoldover >= deps[i].lineupDur + deps[i].taxiOutDur + slackDur[i];

  	CompleteOrderInSeparationIdenticalFlights:
  		forall (<i, j> in SeparationIdenticalFlightPairs)
  		  	scheduledTime[j] >= scheduledTime[i] + sep[i, j];

  	CompleteOrderFromDisjointWindows:
	  	forall (<i, j> in DisjointWindowConsecutivePairs)
	  	  	scheduledTime[j] >= scheduledTime[i] + sep[i, j];

  	SeparationInOverlappingWindowFlights:
  		forall (<i, j> in OverlappingWindowFlightPairs)
  		  	scheduledTime[j] >= scheduledTime[i]
		  		+ sep[i, j] * (scheduledTime[j] >= scheduledTime[i] + 1)
		  		- (latestTime[i] - earliestTime[j]) * (scheduledTime[i] >= scheduledTime[j] + 1);
};
