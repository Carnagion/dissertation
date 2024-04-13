#import "@preview/cetz:0.2.2": canvas, chart, draw, palette, plot
#import "@preview/lovelace:0.2.0": algorithm, pseudocode-list, setup-lovelace
#import "@preview/timeliney:0.0.1": timeline

// NOTE: Needs to be called at the top of the document to setup lovelace
#show: setup-lovelace

#set text(font: "EB Garamond", size: 11pt)
#set par(justify: true)

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#set math.equation(numbering: "(1)")

// NOTE: Hack for fine-grained equation numbering - see https://github.com/typst/typst/issues/380 and https://github.com/typst/typst/issues/380#issuecomment-1523884719
#let multi-equation(equations) = {
    let reduce(array, f) = array.slice(1).fold(array.first(), f)
    let concat(array) = reduce(array, (acc, elem) => acc + elem)

    if equations.has("children") {
        let children = equations.children.filter(child => child != [ ])

        let body-or-children(equation) = if equation.body.has("children") {
            concat(equation.body.children)
        } else {
            equation.body
        }

        let hide-equation(equation) = if equation.has("numbering") and equation.numbering == none {
            math.equation(block: true, numbering: none, hide(equation))
        } else [
            $ #hide(body-or-children(equation)) $ #if equation.has("label") { equation.label }
        ]

        let hidden = box(concat(children.map(hide-equation)))

        let align-equations(acc, equation) = acc + if acc != [] { linebreak() } + equation
        let aligned = math.equation(block: true, numbering: none, children.fold([], align-equations))

        // NOTE: Spacing needs to be explicitly set to exactly this value for the hack to work
        show math.equation: set block(spacing: 0.65em)

        hidden
        style(style => v(-measure(hidden, style).height, weak: true))
        aligned
    }
}

// NOTE: Workaround to get non-math text to use EB Garamond in math equations until Typst ships a native function for doing so
#let mathtext = math.text.with(font: "EB Garamond", weight: "regular")

#let pseudocode-list = pseudocode-list.with(indentation-guide-stroke: 0.2pt)

#set figure(gap: 1em)
#show figure.caption: caption => {
    set text(size: 10pt)
    strong(caption.supplement)
    [ ]
    context strong(caption.counter.display(caption.numbering))
    [: ]
    caption.body
}

#show figure: set block(spacing: 2em)
#show figure.where(kind: table): set block(breakable: true)
#show figure.where(kind: table): set par(justify: false)

#set table(align: center + horizon, stroke: none)
#set table.header(repeat: false)
#show table.cell.where(y: 0, rowspan: 1): strong
#show table.cell: set text(size: 10pt)

// NOTE: Workaround to make prose citations use "et al" with a lower author count threshold until Typst either fixes
//       the default IEEE style or provides a better way of customising this
#show cite.where(form: "prose"): set cite(style: "ieee-et-al-min.csl")

// TODO: Remove when all todos are removed
#let todo(message) = raw("// TODO: " + message, block: true, lang: "rust")

#v(1fr)
#align(center)[
    // TODO: Should the heading be changed to something better?
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    // TODO: Check if "Final Dissertation" is what it should be called
    #text(size: 14pt)[_COMP3003 Final Dissertation_]

    #v(0.2fr)

    #let email(email) = link("mailto:" + email, raw(email))

    // TODO: Remove name before submitting
    #stack(dir: ltr, spacing: 1fr)[
        _By_\
        Indraneel Mahendrakumar\
        20372495\
        #email("psyim3@nottingham.ac.uk")\
    ][
        _Supervised By_\
        Geert De Maere\
        Assistant Professor\
        #email("geert.demaere@nottingham.ac.uk")\
    ]

    #v(0.2fr)

    MSci. Hons. Computer Science with Artificial Intelligence\
    University of Nottingham\
    2023-2024
]
#v(1fr)

#pagebreak()

#v(1fr)

#align(center)[
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 14pt)[*Abstract*]
]

#todo("Write abstract")

#v(1fr)

#pagebreak()

#outline(indent: auto)
#pagebreak()

// NOTE: Done after cover page, abstract, and table of contents since we don't want page numbers to show up on them
#set page(numbering: "1")

// NOTE: Forces the page numbering to begin from here rather than from the cover page or abstract page
#counter(page).update(1)

// TODO: Revise all headings as necessary

= Introduction

#todo("Write introduction")

== Background

#todo("Write background")

== Motivation

#todo("Write about motivation")

= Existing Literature

#todo("Write short introduction to existing literature and past approaches")

== Approximate Methods

#todo("Write short introduction to approximate methods")

=== Heuristics

#cite(<bianco-minimizing-time>, form: "prose") propose two heuristic approaches -- Cheapest Addition Heuristic (CAH) and Cheapest Insertion Heuristic (CIH) -- that each generate sequences by either appending remaining aircraft to partial sequences, or inserting them.
They note that the latter almost always performed better than the former, as it searches for the best partial sequences obtained by inserting new aircraft anywhere within the sequence as opposed to just at the end.
However, it is also much more computationally expensive -- these heuristics are shown to have computational complexities of $O(n^2 log(n))$ and $O(n^4)$ respectively @bianco-minimizing-time @bennell-runway-scheduling.
The heuristics are evaluated on mainly randomly generated test instances.

#cite(<atkin-hybrid-metaheuristics>, form: "prose") introduce a hybridised approach using different search methods and metaheuristics to solve the integrated runway scheduling and ground movement problem at London Heathrow.
They consider total delay, CTOT compliance, positional delay, and throughput as part of their objective function.
Their results show that the availability of more information about aircraft taxiing from the integration of the problem can reduce delays and CTOT violations.

== Exact Methods

#todo("Write short introduction to exact methods")

=== Mathematical Programming

Many existing mixed-integer programming (MIP) approaches to runway sequencing view the problem as a variant of classical machine scheduling problem with sequence-dependent setup times -- the runways are the machines, flights correspond to jobs, and runway separations to setup times @avella-time-indexed.
The literature on machine scheduling has traditionally comrpised of two main kinds of formulations -- big-$M$ and time-indexed @avella-time-indexed.

Big-$M$ formulations represent the landing or take-off time of an aircraft as a single continuous variable.
However, such formulations typically need to apply a heuristic decomposition of the problem in order to meet computation time limits for instances of practical interest @avella-time-indexed.

#cite(<beasley-scheduling-aircraft>, form: "prose") introduce a linear programming (LP)-based tree search approach for the problem of sequencing arrivals on a single runway, and later extend their formulation to handle multiple runways as well.
They consider hard time window constraints and use an objective function that penalises landing before or after a given target time for each arrival.
Unlike many previous approaches that assumed an indefinite latest time limit for landing, their approach employs more realistic latest landing times based on fuel considerations.
This allows for simplifying the problem by exploiting the presence of increased disjoint intervals, caused by relatively narrow hard time windows.

In contrast to big-$M$ formulations, time-indexed formulations discretise the overall time horizon in small time periods.
The schedule of an aircraft is modelled by a set of binary decision variables, only one of which will be 1 -- identifying the aircraft's landing or take-off time -- in any feasible solution.
In general, time-indexed formulations return much stronger bounds than big-$M$ formulations, but at the cost of increasing the number of variables and constants, and consequently computation times @avella-time-indexed.
This makes them often unattractive for real-time applications of runway sequencing, where there are stringent limits on the computation times @bennell-runway-scheduling.

#cite(<avella-time-indexed>, form: "prose") counter this claim by presenting a time-indexed MIP formulation based on a novel class of valid clique inequalities for the single machine scheduling problem with sequence-dependent setup times.
They generalise a family of inequalities introduced by #cite(<nogueira-mixed-integer>, form: "prose").
Their formulation significantly improve the quality of the lower bounds, reduces the number of constraints, and is capable of solving difficult instances from large airports in Europe, namely Stockholm Arlanda, Hamburg, and Milano Linate.

#cite(<beasley-scheduling-aircraft>, form: "prose") also present an alternative time-indexed 0-1 MIP formulation that can be derived by discretising time, although they note that this formulation produces a relatively large number of variables and constraints, and do not explore it further.

=== Dynamic Programming

Dynamic programming (DP) is a general optimisation technique for making sequential decisions.
There have been several attempts to develop efficient DP algorithms for runway sequencing, since it is known to work well for runway sequencing -- almost all runway sequencing problems can be modelled as DP problems as DP algorithms can evaluate partial sequences independently of the exact sequencing decisions taken to generate them @bennell-runway-scheduling.
DP can also yield optimal solutions significantly faster than MIP solvers @lieder-dynamic-programming.

#cite(<psaraftis-dynamic-programming>, form: "prose") proposes a DP algorithm for the single runway scheduling problem, considering runway utilisation and total delay as an objective function.
He utilises an approach that groups aircraft into multiple classes or sets, essentially merging lists of aircraft from these sets and allowing the exploitation of known precedence relations within them.
When implemented as a pre-processing step, this DP algorithm has a time complexity to $O(m^2 (n + 1)^m)$, where $n$ denotes the number of aircraft, and $m$ denotes the number of distinct aircraft types @psaraftis-dynamic-programming @demaere-pruning-rules.

#cite(<balakrishnan-runway-operations>, form: "prose") introduce an alternative DP approach wherein the runway sequencing problem is formulated as a modified shortest path problem in a network, considering positional equity (via maximum shift constraints), minimum separation requirements, precedence constraints, and time window constraints.
Their proposed algorithm has a complexity of $O(n (2k + 1)^(2k + 2))$, where $n$ is the number of aircraft and $k$ is the maximum shift parameter. 

=== Branch-and-Bound

Branch-and-bound is an exact search method for solving optimisation problems by breaking them down into smaller sub-problems and eliminating those sub-problems that cannot possibly contain a solution better than the best known solution so far.
Branch-and-bound algorithms for minimisation problems typically comprise of four main procedures -- separation, bounding, branching, and fathoming @luo-branch-bound.
The use of a bounding function to eliminate sub-problems allows the algorithm to prune nodes from the search space and perform better than a brute-force (exhaustive) search, while still exploring every node in the search space.

#cite(<abela-optimal-schedules>, form: "prose") propose a branch-and-bound algorithm based on a 0-1 MIP formulation of the runway sequencing problem for arrivals on a single runway, considering separation and precedence constraints and using an objective function that penalises speeding up or holding the arrivals.
They compare the branch-and-bound algorithm against a genetic algorithm based on the same formulation, evaluating both on randomly generated test data with up to 20 aircraft.
Results show that their branch-and-bound implementation solves smaller problem instances relatively quickly, but requires considerably longer time as instances grow larger.

#cite(<ernst-heuristic-exact>, form: "prose") present a branch-and-bound algorithm for the problem of sequencing arrivals with single as well as multiple runways, using an objective function that penalises landing before or after target times.
They develop a specialised simplex algorithm capable of determining landing times rapidly and a heuristic algorithm to obtain the upper and lower bounds for the branch-and-bound algorithm.
Furthermore, they utilise a variety of pre-processing methods to improve the efficiency of the algorithm, and evaluate both their heuristic and their branch-and-bound approaches on instances involving up to 50 aircraft.

== Paradigms

#todo("Write short introduction to paradigms to improve tractability")

=== Constrained Position Shifts

A number of approaches in the past -- such as that of #cite(<psaraftis-dynamic-programming>, form: "prose") and #cite(<balakrishnan-runway-operations>, form: "prose") -- have employed Constrained Position Shifting (CPS), a technique that was first introduced by #cite(<dear-dynamic-scheduling>, form: "prose").
CPS restricts an aircraft's maximum shift in position relative to its original position in some initial sequence, which is typically obtained using a FCFS approach.
Not only does this prune the search space by reducing the number of aircraft that must be considered for each position in the sequence, but it also enforces positional equity by preventing aircraft from being advanced or delayed disproportionately compared to other aircraft @dear-dynamic-scheduling @demaere-pruning-rules.

Although CPS can be an effective and efficient approach in many cases of arrival sequencing, delays may differ widely between arrivals and departures in mixed-mode operations, making maximum position shift constraints impractical @demaere-pruning-rules.

#cite(<atkin-tsat-allocation>, form: "prose") further show that when CTOT slots are considered, CTOT compliance and positional equity are heavily in conflict -- there is a tradeoff between the number of CTOT violations and positional equity.
Moreover, having a hard constraint of or high penalty for positional equity may be highly counter-productive for take-offs even apart from its conflict with delay or CTOT compliance @atkin-tsat-allocation.

For instance, there may be an aircraft that must wait for the start of its CTOT slot, during which other aircraft may be sequenced with no additional delay -- however, penalising positional inequity would (wrongfully) penalise such a sequence, forcing the other aircraft to take off after the one with the CTOT slot and increasing the overall delay in the process @atkin-tsat-allocation.

The differing delays that accumulate across different Standard Instrument Departure (SID) routes, hard time window constraints, and CTOT constraints can thus require large maximum position shifts to obtain good runway sequences, thereby challenging the tractability of CPS-based approaches @demaere-pruning-rules.
The model and branch-and-bound program presented in @section:model and @section:branch-bound of this paper therefore do not employ CPS, making them more practical and viable for real-world scenarios considering departures with complex separation requirements and CTOT compliance.

=== Pruning Rules

In contrast to approaches like CPS that reduce the search space of the problem by limiting the positional shifting of aircraft within the sequence, pruning rules exploit the characteristics of the problem or objective function to infer that a current sequence (or any future sequences based on it) is sub-optimal.
This has the advantage of being able to prune partial subsequences that show known poor characteristics much earlier, even before dominating partial sequences have been generated @demaere-pruning-rules.

Pruning rules have been extensively studied in the literature involving machine scheduling @demaere-pruning-rules @allahverdi-survey-scheduling.
However, #cite(<allahverdi-survey-scheduling>, form: "prose") show that a majority of these approaches do not consider sequence-dependent setup times, despite them being prevalent in runway sequencing problems and in many other applications of machine scheduling.
Nor do they consider complex non-linear, non-convex, or discontinuous objective functions @demaere-pruning-rules.

#cite(<demaere-pruning-rules>, form: "prose") introduce a set of pruning principles that exploit the inherent characteristics of the runway sequencing problem including complete orders, conditional orders, insertion dominance, and dominance with lower bounding.
Their pruning rules enable significant reductions of the problem's computational complexity without compromising the optimality of the generated solutions, and are usually much more computationally efficient compared to pruning rules based on local improvements @demaere-pruning-rules.

Furthermore, they show that many of the pruning rules considered transfer to other objective functions commonly considered in the literature, and can thus be used outside of the specific DP approach developed by them @demaere-pruning-rules.
A subset of these pruning rules is thus incorporated into the model presented in this paper to improve its tractability.

=== Rolling Horizons

#todo("Write about rolling horizon approaches")

= Problem Description

Given a set of arrivals $A$ and departures $D$, the runway and de-icing sequencing problem for a single runway and single de-icing station consists of finding a sequence of landing and take-off times as well as a sequence of de-icing times such that an optimal value is achieved for a given objective function, subject to the satisfaction of all hard constraints.

== Notation

@table:notation provides an overview of the symbols used in the following sections along with their definitions.

#let notation = table(
    columns: 2,
    stroke: (x, y) => (
        top: if y == 1 { (dash: "solid", thickness: 0.5pt) },
        left: if x == 1 { (dash: "solid", thickness: 0.5pt) },
    ),
    align: (x, y) => if x > 0 and y > 0 { left + horizon } else { center + horizon },
    table.header[Symbol][Definition],
    $A$, [Set of arrivals],
    $D$, [Set of departures],
    $F$, [Set of aircraft, defined as $A union D$],
    $b_i$, [Base landing or take-off time for aircraft $i$],
    $p_i$, [Pushback duration for departure $i$],
    $m_i$, [Duration to taxi from gates to de-icing station for departure $i$],
    $o_i$, [De-icing duration for departure $i$],
    $n_i$, [Taxi-out duration for departure $i$],
    $q_i$, [Lineup duration for departure $i$],
    $h_i$, [Maximum holdover duration for departure $i$],
    $w_i$, [Maximum runway hold duration for departure $i$],
    $u_i$, [Start of CTOT slot for departure $i$],
    $v_i$, [End of CTOT slot for departure $i$],
    $e_i$, [Start of hard time window for aircraft $i$],
    $l_i$, [End of hard time window for aircraft $i$],
    $delta_(i, j)$, [Minimum separation between aircraft $i$ and $j$, where $i$ precedes $j$],
    $r_i$, [Release time for aircraft $i$],
    $d_i$, [Due time for aircraft $i$],
    $T_i$, [Set of possible landing or take-off times for aircraft $i$],
    $Z_i$, [Set of possible de-icing times for departure $i$],
    $tau_(i, t)$, [Boolean decision variable indicating if an aircraft $i$ is scheduled to land or take off at time $t$],
    $zeta_(i, z)$, [Boolean decision variable indicating if a departure $i$ is scheduled to de-ice at time $z$],
    $gamma_(i, j)$, [Boolean decision variable indicating whether an aircraft $i$ lands or takes off before an aircraft $j$],
    $t_i$, [Scheduled landing or take-off time for aircraft $i$],
    $z_i$, [Scheduled de-ice time for departure $i$],
    $f(s)$, [Objective value for partial or final sequence $s$],
    $c_d (i)$, [Delay cost for aircraft $i$],
    $c_v (i)$, [CTOT violation cost for departure $i$],
    $F_S$, [Set of pairs of distinct aircraft $(i, j)$ with disjoint hard time windows such that $e_j >= l_i + delta_(i, j)$],
    $F_D$, [Set of pairs of distinct aircraft $(i, j)$ with disjoint hard time windows such that $e_j < l_i + delta_(i, j)$],
    $F_O$, [Set of pairs of distinct aircraft $(i, j)$ with overlapping hard time windows],
    $F_C$, [Set of pairs of distinct separation-identical aircraft $(i, j)$ with a complete order such that $i$ lands or takes off before $j$],
)

#figure(
    notation,
    caption: [Overview of notation and definitions used in model],
) <table:notation>

== Constraints

A feasible solution to the problem must satisfy runway precedence, separation requirements, base times, hard time windows, CTOT slots, holdover times, and runway hold times.
A sequence that violates these hard constraints is considered to be infeasible, and can thus be eliminated from the solution space.

=== Precedences

Since this is a single runway formulation, no two aircraft can land or take off at the same time.
Let $gamma_(i, j)$ be a boolean decision variable indicating whether aircraft $i$ lands or takes off before aircraft $j$.
The following constraint can then be imposed on every pair of distinct aircraft $(i, j)$:

$ gamma_(i, j) + gamma_(j, i) = 1 $

That is, either $i$ must land or take off before $j$, or the other way around.
Similar precedence constraints exist for de-icing -- given any two distinct departures $i$ and $j$, either $i$ must finish its de-icing before $j$ can start de-icing, or the other way around:

$ z_j >= z_i + o_i or z_i >= z_j + o_j $

=== Runway Separations

Any two consecutive aircraft $i$ and $j$ (where $i$ precedes $j$) are required to have a minimum _runway separation_ $delta_(i, j)$ between them, which is determined by their weight classes, speed groups, and (for departures) Standard Instrument Departure (SID) routes.
An aircraft's weight class influences the severity of wake turbulence it causes, the time required for this turbulence to dissipate, and its sensitivity to the wake turbulence caused by other aircraft.
Larger or heavier aircraft typically generate greater turbulence, to which smaller or lighter aircraft are more sensitive.
Consequently, a larger separation may be required when a large aircraft is followed by a small one, than when a small aircraft is followed by a large one @demaere-pruning-rules.

Similarly, a larger separation may be required when a slow aircraft is followed by a faster one on the same route, to prevent the latter from catching up to the former before their routes diverge.
Separations for SID routes are also influenced by the climb and relative bearing of the route, as well as congestion in downstream airspace sectors.
The latter factor may require an increased separation upon take-off to space out traffic and prevent the overloading of en-route sectors and controllers @demaere-pruning-rules.

The minimum separation that must be maintained between two aircraft is thus the maximum of the separations due to their weight classes, speed groups, and SID routes.
The required separations between each ordered pair of distinct aircraft can therefore be expressed as a separation matrix @demaere-pruning-rules.

However, runway separations do not necessarily obey the _triangle inequality_ -- i.e. for any three aircraft $i$, $j$, and $k$, the inequality $delta_(i, j) + delta_(j, k) >= delta_(i, k)$ is not necessarily true @demaere-pruning-rules.
An aircraft's landing or take-off time can thus be influenced by not just the immediately preceding aircraft, but by multiple preceding aircraft.

=== Base Times

Every aircraft has an earliest possible landing or take-off time -- henceforth referred to as its _base time_ -- which is defined as the time the aircraft enters the runway queue and finishes lining up (for departures), or the local airspace (for arrivals).
The base time $b_i$ of an aircraft $i$ is modelled as a hard constraint -- i.e. $i$ cannot be scheduled to land or take off before $b_i$:

$ t_i >= b_i $

=== Time Windows

If an aircraft $i$ is subject to a hard time window which is defined by its earliest (start) time $e_i$ and latest (end) time $l_i$, then its landing or take-off time $t_i$ must be within this window:

$ e_i <= t_i <= l_i $

In this model, each aircraft is assumed to be subject to a hard time window, although this is not always the case in the real world.
However, this assumption can be made without loss of generality -- an aircraft $i$ that is not subject to a hard time window can instead be considered to be subject to a very large time window, such that its start time $e_i$ is early enough and its end time $l_i$ late enough so as to never affect solutions in practice @demaere-pruning-rules.

=== Calculated Take-Off Times

In addition to a hard time window, a departure $i$ might be subject to a Calculated Take-Off Time (CTOT) slot, during which it should take off.
Typically, a CTOT has a tolerance of -5 to +10 minutes (i.e. five minutes before and ten minutes after $c_i$) and its time window can thus be defined by its earliest (start) time $u_i$ and latest (end) time $v_i$; however, this model allows for customizable CTOT tolerances per departure.

Much like a hard time window, a departure cannot take off before $u_i$, but it may be scheduled after $v_i$ -- although this is heavily penalised.
The start time of a CTOT slot is thus modelled as a hard constraint, while its end time is modelled as a soft constraint:

$ t_i >= u_i $

=== Holdover Times

Once a departure $i$ has been de-iced, the applied de-icing fluid will remain effective for a certain duration of time, called the Holdover Time (HOT) $h_i$.
Departures must take off within this period of time -- if a departure's HOT expires before it takes off, it must be de-iced again, which could extend the de-icing queue and delay subsequent aircraft.

The HOT of a departure $i$ is thus modelled as a hard constraint -- the time between its de-ice time $z_i$ and take-off time $t_i$ must not be greater than $h_i$:

$ t_i - z_i - o_i <= h_i $

=== Runway Hold Times

// TODO: Write a better explanation for this section
Delays are ideally absorbed by stand holding -- a departure $i$ only needs to push back only when absolutely necessary to meet its de-ice time $z_i$ (if applicable) and take-off time $t_i$.

However, in some cases it may be better to absorb delays at the runway by _runway holding_ instead -- i.e. arriving and waiting at the runway before a departure's scheduled take-off time.
A departure that pushes back earlier than absoltuely necessary would be able to de-ice earlier than necessary, freeing up the de-icing queue earlier.
This could in turn enable the following departures to de-ice earlier and potentially reduce the total delay and CTOT violations in the remaining sequence.

The maximum runway holding duration $w_i$ for a departure $i$ is thus modelled as a hard constraint -- the time between $z_i$ and $t_i$ must not be greater than the sum of its de-ice duration $o_i$, post de-ice taxi duration $n_i$, lineup duration $q_i$, and maximum runway holding duration $w_i$:

$ t_i - z_i <= o_i + n_i + q_i + w_i $

== Objectives

The objective function $f(s)$ for a partial or final sequence $s$ is defined in @eq:objective-function.
It considers total delay and CTOT compliance, and is based on the function described by #cite(<demaere-pruning-rules>, form: "prose").

=== Runway Utilisation

The runway utilisation of a partial or final sequence $s$ is modelled as the _makespan_ of $s$, i.e. $max_(i in s) t_i$.
Although not directly included as an objective, it is utilised for the evaluation of partial sequences generated by the branch-and-bound program and their subsequent pruning according to the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose").

=== Delay

The delay for an aircraft $i$ is defined as the difference between its landing or take-off time $t_i$ and its base time $b_i$.
Its delay cost $c_d (i)$, defined in @eq:delay-cost, is then calculated as the delay squared, and is equivalent to the following function:

$ c_d (i) = (t_i - b_i)^2 $

Raising the delay cost to a power greater than one penalises disproportionately large delays more severely and encourages a more equitable distribution of delay across all aircraft @demaere-pruning-rules.
For instance, two aircraft with delays of one and three minutes each would have a total delay cost of $1^2 + 3^2 = 10$, whereas the same two aircraft with delays of two minutes each would have a total delay cost of only $2^2 + 2^2 = 8$, making the latter more preferable.

=== Calculated Take-Off Time Compliance

The CTOT violation cost $c_v (i)$ for a departure $i$ is defined in @eq:ctot-violation-cost, and is equivalent to the following piecewise discontinuous non-linear function given by 0 if it takes off within its CTOT slot and the squared difference between its take-off time $t_i$ and its CTOT slot end time $v_i$ if it misses its CTOT slot:

$ c_v (i) = cases(
    &0 &"if" &u_i <= t_i <= v_i,
    &(t_i - v_i)^2 &"if" &t_i > v_i,
) $

== Model <section:model>

A time-indexed formulation is employed in order to linearise the objective function and hence solve the integrated runway and de-icing sequencing problem using 0-1 integer linear programming.

First, the landing or take-off time of an aircraft $i$ is constrained to lie between the earliest possible time $i$ can be scheduled to land or take off -- its _release time_ $r_i$ and the latest possible time $i$ can be scheduled to land or take off -- its _due time_ $d_i$.
The release time of $i$ be calculated as the maximum of its base time $b_i$, start time $e_i$ of its hard time window, and start time $u_i$ of its CTOT slot (if applicable):

$ r_i = max(b_i, e_i, u_i) $

Meanwhile, the due time of $i$ is simply the end time $l_i$ of its hard time window.
A feasible runway sequence will always schedule $i$ at a time between $r_i$ and $d_i$.
The set of possible landing or take-off times $T_i$ for an aircraft $i$ can thus be defined as the set of all times between $r_i$ and $d_i$:

$ T_i = { r_i, ..., d_i } $

A binary decision variable $tau_(i, t)$ is then associated with every aircraft $i in F$ and every time $t in T_i$, which is $1$ if and only if $i$ lands or takes off at time $t$.
Since every aircraft is assigned exactly one time to land or take off, the following constraint must hold for all $i in F$:

$ sum_(t in T_i) tau_(i, t) = 1 $

Similarly, the set of possible de-icing times $Z_i$ for a departure $i$ -- irrespective of when it takes off -- can be defined as the set of times between its earliest possible de-icing time and latest possible de-icing time:

$ Z_i = { r_i - q_i - w_i - n_i - o_i, ..., d_i - q_i - n_i - o_i } $

A binary decision variable $zeta_(i, z)$ can then be associated with every departure $i in D$ and every time $z in Z_i$, which is $1$ if and only if $i$ starts de-icing at time $z$.
Much like the landing or take-off time, every departure $i$ is assigned exactly one time to de-ice:

$ sum_(z in Z_i) zeta_(i, z) = 1 $

Putting together these constraints, objectives, and time-indexed formulations, a 0-1 integer linear model for the integrated runway and de-icing sequencing problem is presented below:

#multi-equation[
    $ "Minimise" space &f(s) = sum_(i in s) c_d (i) + c_v (i) $ <eq:objective-function>
    $ &c_d (i) = sum_(t in T_i) tau_(i, t) dot (t - b_i)^2 &forall i in F $ <eq:delay-cost>
    $ &c_v (i) = sum_(t in T_i) tau_(i, t) dot (t > v_i) dot (t - v_i)^2 &forall i in D $ <eq:ctot-violation-cost>
    $ &t_i = sum_(t in T_i) tau_(i, t) dot t &forall i in F $ <eq:scheduled-time>
    $ &z_i = sum_(z in Z_i) zeta_(i, z) dot z &forall i in D $ <eq:deice-time>
    $ "Subject to" space &sum_(t in T_i) tau_(i, t) = 1 &forall i in F $ <constraint:schedule-once>
    $ &sum_(z in Z_i) zeta_(i, z) = 1 &forall i in D $ <constraint:deice-once>
    $ &gamma_(i, j) + gamma_(j, i) = 1 &forall i in F, j in F, i != j $ <constraint:schedule-precedence>
    $ &z_j >= z_i + o_i or z_i >= z_j + o_j &forall i in D, j in D, i != j $ <constraint:deice-precedence>
    $ &t_i >= z_i + o_i + n_i + q_i &forall i in D $ <constraint:min-taxi>
    $ &t_i - z_i - o_i <= h_i &forall i in D $ <constraint:max-holdover>
    $ &t_i - z_i - o_i <= n_i + w_i + q_i &forall i in D $ <constraint:max-runway-hold>
    $ &gamma_(i, j) = 1 &forall (i, j) in F_S union F_D union F_C $ <constraint:certain-precedence>
    $ &t_j >= t_i + delta_(i, j) &forall (i, j) in F_D union F_C $ <constraint:certain-separation>
    $ &t_j >= t_i + delta_(i, j) dot gamma_(i, j) - (d_i - r_j) dot gamma_(j, i) &forall (i, j) in F_O $ <constraint:overlapping-window-separation>
    $ &tau_(i, t) in { 0, 1 } &forall i in F, t in T_i $ <constraint:schedule-binary>
    $ &zeta_(i, z) in { 0, 1 } &forall i in D, z in Z_i $ <constraint:deice-binary>
    $ &gamma_(i, j) in { 0, 1 } &forall i in F, j in F, i != j $ <constraint:precedence-binary>
]

The objective function -- @eq:objective-function -- minimises total delay and CTOT violations, whose individual costs are given by @eq:delay-cost and @eq:ctot-violation-cost respectively.
The individual cost functions $c_d (i)$ and $c_v (i)$ are linearised according to the time-indexed formulations described above.

@eq:scheduled-time and @eq:deice-time define the scheduled landing or take-off time and the de-ice time (if applicable) for an aircraft.

@constraint:schedule-once ensures that every aircraft is assigned exactly one landing or take-off time within its time window, and @constraint:deice-once ensures that every departure that must de-ice is assigned a de-ice time within its de-ice time window.

@constraint:schedule-precedence enforces precedence constraints for every aircraft -- either $i$ must land or take off before $j$, or the other way around.

@constraint:deice-precedence enforces de-icing precedence constraints for every departure, and ensures that a departure can only begin de-icing after the current aircraft (if any) has finished being de-iced.

@constraint:min-taxi ensures that a departure has enough time to taxi out after it finishes de-icing and lineup at the runway to meet its scheduled take-off time.

@constraint:max-holdover ensures that the time between a departure's scheduled take-off time and de-ice time does not exceed its allowed HOT -- i.e. once de-iced, departures take off before their HOT expires.

@constraint:max-runway-hold ensures that the runway holding time of a departure does not exceed its maximum allowed runway holding time.

@constraint:certain-precedence, @constraint:certain-separation, and @constraint:overlapping-window-separation enforce precedence and separation constraints on all pairs of distinct aircraft.
These constraints are inferred from disjoint time windows as well as complete orders in separation-identical aircraft, which are discussed further in @section:disjoint-time-windows and @section:complete-orders respectively.

@constraint:schedule-binary, @constraint:deice-binary, and @constraint:precedence-binary restrict the decision variables for landings or take-offs, de-icing, and aircraft precedences to binary values.

// TODO: Check if this section looks better elsewhere, such as just after time windows or precedences, or just after the model
=== Disjoint Time Windows <section:disjoint-time-windows>

#cite(<beasley-scheduling-aircraft>, form: "prose") show that it can be determined for certain pairs of distinct aircraft $(i, j)$ whether $i$ lands or takes off before $j$ does, based on their sets of possible landing or take-off times.
For example, if two aircraft $i$ and $j$ have their release times and due times as $r_i = 10$, $d_i = 50$, $r_j = 70$, and $d_j = 110$ respectively, then it is clear that $i$ must land or take off first (i.e. before $j$) since $T_i$ and $T_j$ are disjoint.
On the other hand, if $r_i = 10$, $d_i = 70$, $r_j = 50$, and $d_j = 110$, then it is not always the case that $i$ lands or takes off before $j$ does (or vice-versa).

Additionally, even if the order of $i$ and $j$ can be inferred due to $T_i$ and $T_j$ being disjoint, their separation constraint may not automatically be satisfied @beasley-scheduling-aircraft.
Continuing the former example above with $r_i = 10$, $d_i = 50$, $r_j = 70$, and $d_j = 110$, if the required separation $delta_(i, j) = 15$, then the separation constraint is automatically satisfied regardless of what times $i$ and $j$ are scheduled to land or take off at.
However, if $delta_(i, j) = 25$, then there exist certain landing or take-off times for $i$ and $j$ such that their separation constraint is violated.

From these observations, #cite(<beasley-scheduling-aircraft>, form: "prose") show that it is possible to define three disjoint sets:
1. The set of pairs of distinct aircraft $(i, j)$ for which $i$ definitely lands or takes off before $j$ does, and for which the separation constraint is automatically satisfied
2. The set of pairs of distinct aircraft $(i, j)$ for which $i$ definitely lands or takes off before $j$ does, but for which the separation constraint is not automatically satisfied
3. The set of pairs of distinct aircraft $(i, j)$ for which $i$ may or may not land before $j$ and vice-versa

Let $F_S$, $F_D$, and $F_O$ be the first, second, and third set respectively.
They can then be defined as shown below:

#multi-equation[
    $ F_S = { (i, j) | &d_i < r_j and d_i + delta_(i, j) <= r_j, i in F, j in F, i != j } $ <eq:separated-windows>
    $ F_D = { (i, j) | &d_i < r_j and d_i + delta_(i, j) > r_j, i in F, j in F, i != j } $ <eq:disjoint-windows>
    $ F_O = { (i, j) | &r_j <= r_i <= d_j or r_j <= d_i <= d_j or r_i <= r_j <= d_i or r_i <= d_j <= d_i,\
        &i in F, j in F, i != j } $ <eq:overlapping-windows>
]

It is then possible to impose the following precedence and separation constraints on every pair of distinct aircraft in these sets, corresponding to @constraint:certain-precedence, @constraint:certain-separation, and @constraint:overlapping-window-separation in the model:

#multi-equation[
    $ &gamma_(i, j) = 1 &forall (i, j) in F_S union F_D $
    $ &t_j >= t_i + delta_(i, j) &forall (i, j) in F_D $
    $ &t_j >= t_i + delta_(i, j) dot gamma_(i, j) - (d_i - r_j) dot gamma_(j, i) &forall (i, j) in F_O $
]

// TODO: Check if this section looks better elsewhere, such as just after time windows or precedences, or just after the model
=== Complete Orders <section:complete-orders>

A _complete order_ exists between any two aircraft $i$ and $j$ if the objective value and feasibility of a sequence $s$ containing both $i$ and $j$ cannot be improved by reversing the order of $i$ and $j$ in $s$.
By exploiting complete orders, it is possible to simplify the problem of runway sequencing (or more generally, machine scheduling) to one of interleaving ordered sets of aircraft, always only sequencing the first available aircraft from each set @demaere-pruning-rules.
This enables a reduction in the problem's worst-case computational complexity from $O(n!)$ to $O(m^2 (n + 1)^m)$, where $n$ denotes the number of aircraft, and $m$ denotes the number of distinct aircraft types @psaraftis-dynamic-programming @demaere-pruning-rules.

#cite(<psaraftis-dynamic-programming>, form: "prose") first showed the existence of such complete orders between _separation-identical_ aircraft.
Two distinct aircraft $i$ and $j$ are separation-identical if their mutual separations with respect to every other aircraft $k in F$ are the same -- i.e. $i$ and $j$ are separation-identical if and only if:

$ forall k in F, k != i and k != j and delta_(i, k) = delta_(j, k) and delta_(k, i) = delta_(k, j) $ <eq:are-separation-identical>

Additionally, a complete order may be inferred upon a set of separation-identical aircraft if the complete orders for each of the individual constraints and objectives are consistent within the set.
#cite(<demaere-pruning-rules>, form: "prose") show that this is the case for an objective that considers delay and makespan, even with hard time window constraints -- as long as there is a consistent order between every aircraft's base times, release times, and end times of hard time windows.
A complete order can thus be inferred between two separation-identical aircraft $i$ and $j$ if and only if:

$ b_i <= b_j and r_i <= r_j and l_i <= l_j $ <eq:are-complete-ordered>

However, complete orders cannot be inferred between two separation-identical aircraft if one or both aircraft are subject to a CTOT, due to the piecewise linear, discontinuous, and non-convex nature of the CTOT violation cost function $c_v (i)$ @demaere-pruning-rules.
Thus, in addition to satisfying @eq:are-complete-ordered, neither of the two aircraft must be subject to a CTOT slot.

Following from @eq:are-separation-identical and @eq:are-complete-ordered, it is possible to define the set $F_C$ of pairs of distinct aircraft $(i, j)$ where $i$ and $j$ are separation-identical and have a complete order such that $i$ must land or take off before $j$:

$ F_C = { (i, j) | &(forall k in F, k != i and k != j and delta_(i, k) = delta_(j, k) and delta_(k, i) = delta_(k, j))\
    &and b_i <= b_j and r_i <= r_j and l_i <= l_j, i in F, j in F, i != j } $

The following precedence and separation constraints can thus be imposed on every pair of aircraft $(i, j) in F_C$, corresponding to @constraint:certain-precedence and @constraint:certain-separation in the model:

$ gamma_(i, j) = 1 $
$ t_j >= t_i + delta_(i, j) $

= Implementation

#todo("Write short introduction to different approaches used")

== Mathematical Program

// TODO: Write more about CPLEX implementation if necessary
The model presented in in @section:model has been implemented as a mathematical program in Optimisation Programming Language (OPL) @opl, which is packaged with IBM's ILOG CPLEX Optimisation Studio @cplex.

== Branch-and-Bound Program <section:branch-bound>

// TODO: Explain why Rust is used if necessary
A branch-and-bound algorithm to optimally solve the integrated runway and de-icing sequencing problem as described in @section:model (as well as its decomposed version) is also developed, using a depth-first-search that incrementally builds up sequences by adding one aircraft to the current partial sequence at every step.
The algorithm is implemented in the Rust programming language @rust.

The algorithm begins with no known best sequence, and a best cost $c_"best"$ of infinity.
It maintains a last-in-first-out (LIFO) queue of nodes to visit along with their depths -- the search space.
A node at depth $k$ in the search space corresponds to a partial sequence $s$ with $k$ aircraft.
The queue is initialised with partial sequences containing solely the first aircraft to be sequenced from each ordered set of separation-identical aircraft.

At each step, the most recently added node (sequence) is removed from the back of the queue, and its _lower bound_ is evaluated.
The lower bound for a partial sequence $s$ at depth $k$ consists of two components -- its actual objective value $f(s)$, and a lower bound on the cost of the remaining $(|F| - k)$ aircraft to be sequenced.
The latter can be calculated by sequencing the remaining aircraft from each ordered set of separation-identical aircraft in a FCFS manner, assuming a minimum separation of one minute between each aircraft.
Although using a small separation and an FCFS approach seldom yields an accurate cost, it avoids overshooting the actual objective value and subsequently pruning a potentially optimal sub-sequence.

If the lower bound of the current node is better (smaller) than the objective value of the best known full sequence $s_"best"$, or if no full sequences have been produced yet, the node is _separated_, producing sub-nodes with depth $k + 1$ -- i.e. new partial sequences with a single aircraft appended to the current partial sequence $s$.
Sub-nodes are added to the front of the queue in decreasing order of their objective value $f(s)$.
Since nodes are removed from the front of the queue, this branching procedure is best-first -- i.e. the partial sequence with the best (lowest) objective value is explored first, as it gets added last.

The algorithm terminates when all nodes are _fathomed_ -- i.e. all nodes have either been separated or ignored (due to having worse lower bounds than the best known sequence).
The full branch-and-bound algorithm as described above is shown in @code:branch-bound:

#let branch-bound-code = pseudocode-list[
    + $c <- 0$
    + $c_"best" = infinity$
    + $s <- $ empty sequence
    + $s_"best" <- s$
    + $q <- $ empty stack
    + *for* each $X$ *in* ordered sets of separation-identical aircraft *do*
        + $i <- $ first aircraft in $X$
        + schedule $t_i$ and $z_i$
        + push $(i, 0)$ to $q$
    + *end*
    + *while* $q$ is not empty *do*
        + $(i, k) <- $ pop from $q$
        + *for* each $j$ *in* $s$ from index $k$ onwards *do*
            + $c <- c - c_d (j) - c_v (j)$
            + reset $t_i$ and $z_i$
        + *end*
        + truncate $s$ to length $k$
        + schedule $t_i$ and $z_i$
        + $c' <- c + c_d (i) + c_v (i)$
        + *if* $c' < c_"best"$ *then*
            + $c <- c'$
            + push $i$ to $s$
            + *if* length of $s = $ total number of aircraft *then*
                + $c_"best" <- c$
                + $s_"best" <- s$
            + *else*
                + *for* each $X$ *in* ordered sets of separation-identical aircraft *do*
                    + *if* $X$ has any aircraft that are not in $s$ *do*
                        + $j <- $ first aircraft in $X$ that is not in $s$
                        + push $(j, k + 1)$ to $q$
                    + *end*
                + *end*
            + *end*
        + *else*
            + reset $t_i$ and $z_i$
        + *end*
    + *end*
]

#algorithm(
    branch-bound-code,
    caption: [
        Branch-and-bound algorithm for runway and de-icing sequencing
    ],
) <code:branch-bound>

This algorithm is de-icing approach-agnostic, i.e. it only describes the branch-and-bound procedure itself, not how an individual aircraft is assigned a landing or take-off time or a de-icing time.
The benefit of this is twofold -- first, it allows for generic and easily extendable implementations of de-icing approaches, and second, it allows each de-icing approach to be compared as fairly as possible since the only difference in code comes from scheduling individual aircraft.

The two different de-icing approaches -- decomposed and integrated, wherein the former is further split into decomposed by TOBT and decomposed by CTOT -- are discussed further in the sections below.

=== Decomposed De-Icing

Under decomposed de-icing, a de-icing queue is first generated before performing the main branch-and-bound procedure.
This queue is then used to fix the de-icing times of departures before scheduling their take-off times.

To generate the de-icing queue, a list of all departures to be de-iced is first created and then sorted by either their TOBTs or their CTOT slots, depending on which decomposed de-icing approach is to be used.
In the latter case, additional sorting may be necessary to prioritise aircraft with CTOT slots over those without and ensure maximal CTOT compliance.
Aircraft in this list may then be assigned de-icing times in a FCFS manner, taking into account their release times and the time that the previous aircraft finishes de-icing.
The de-icing time $z_i$ of a departure $i$ within the de-icing queue $Q$ is thus given by the following expression, where $Q_i$ is the set of departures that have been scheduled to de-ice before $i$:

$ z_i = max(r_i - q_i - n_i - o_i, r_i - h_i - o_i, max_(j in Q_i) z_j + o_j) $ <eq:deice-queue>

Following from @eq:deice-queue, the take-off time $t_i$ of a departure $i$ that has already been scheduled to de-ice can be calculated as the maximum of its release time $r_i$, de-icing time $z_i$ plus the time taken to taxi to the runway after de-icing, and $t_j + delta_(j, i)$ for every $j in s_i$, where $s_i$ is the partial sequence of all aircraft that have been sequenced before $i$:

$ t_i = max(r_i, z_i + o_i + n_i + q_i, max_(j in s_i) t_j + delta_(j, i)) $

On the other hand, if $i$ is an aircraft which is either an arrival or a departure that is not required to de-ice, its landing or take-off time does not need to consider the second component, and can simply be given by:

$ t_i = max(r_i, max_(j in s_i) t_j + delta_(j, i)) $ <eq:earliest-landing>

=== Integrated De-icing

Unlike its decomposed counterpart, integrated de-icing does not calculate a de-icing queue beforehand, but instead calculates the de-icing times for aircraft along with their landing or take-off times.

If an aircraft $i$ is an arrival or a departure that is not required to de-ice, then its landing or take-off time $t_i$ can be calculated as given by @eq:earliest-landing.
However, if $i$ is a departure and is required to de-ice before taking off, then its earliest de-icing time must also be taken into consideration.
This can be calculated as the time that the preceding aircraft in the de-icing queue finishes de-icing, plus the departure $i$'s de-icing duration $o_i$, taxi-out duration $n_i$, and lineup duration $q_i$.
Thus, the take-off time $t_i$ for a departure $i$ that must de-ice is given by:

$ t_i = max(r_i, max_(j in s_i) t_j + delta_(j, i), max_(k in s_i) z_k + o_i + n_i + q_i) $ <eq:earliest-takeoff>

Once its take-off time is known, its de-icing time $z_i$ can be calculated as the maximum of its earliest de-icing time (given above in @eq:earliest-takeoff), $t_i$ minus the sum of its HOT $h_i$ and de-icing duration $o_i$, and $t_i$ minus the sum of its maximum allowed runway hold duration $r_i$, lineup duration $q_i$, taxi-out duration $n_i$, and de-icing duration $o_i$:

$ z_i = max(t_i - q_i - r_i - n_i - o_i, t_i - h_i - o_i, max_(j in s_i) z_j + o_i + n_i + q_i) $

=== Rolling Horizon Extension

#todo("Include explanation and pseudocode for rolling horizon")

// TODO: Check if this belongs here or is better off somewhere else
== Sequence Visualiser

#todo("Write about visualiser")

= Results

#todo("Write introduction to results")

#let results-table(group-headers: (), side-headers: false, ..datasets) = {
    let header-groups = datasets
        .pos()
        .map(array.first)
    
    let data-groups = datasets
        .pos()
        .map(data-group => data-group.slice(1))

    let side-header = ()
    let side-data = ()
    let data = array.zip(..data-groups).flatten()
    if side-headers {
        side-header = (table.cell(rowspan: 2, header-groups.first().first()),)
        side-data = data-groups.first().map(array.first)

        header-groups = header-groups.map(header-group => header-group.slice(int(side-headers)))
        data-groups = data-groups.map(data-group => data-group.map(row => row.slice(int(side-headers))))

        data = side-data.zip(..data-groups).flatten()
    }

    data = data.map(str.trim).map(table.cell)

    let headers = header-groups
        .flatten()
        .map(str.trim)
        .map(table.cell)

    let group-headers = group-headers
        .zip(header-groups.map(array.len))
        .map(pair => {
            let (group-header, headers-len) = pair
            table.cell(colspan: headers-len, group-header)
        })

    let header-rowspan = if group-headers == () { 1 } else { 2 }
    let header-colspan = header-groups.first().len()
    
    let stroke = (x, y) => {
        let stroke-style = (dash: "solid", thickness: 0.5pt)
        let h = if y > 0 and y <= header-rowspan {
            stroke-style
        }
        let v = if x != 0 and calc.rem-euclid(x, header-colspan) == side-header.len() {
            stroke-style
        }
        (
            top: h,
            bottom: none,
            left: v,
            right: none,
        )
    }

    table(
        columns: side-header.len() + headers.len(),
        stroke: stroke,
        table.header(
            ..side-header,
            ..group-headers,
            ..headers,
        ),
        ..datasets.named(),
        ..data,
    )
}

#let results = (
    furini: (
        branch-bound: (
            decomposed: csv("results/furini/branch-bound/deice-decomposed.csv"),
            integrated: csv("results/furini/branch-bound/deice-integrated.csv"),
        )
    ),
    heathrow: (
        branch-bound: (
            tobt: csv("results/heathrow/branch-bound/deice-tobt.csv"),
            ctot: csv("results/heathrow/branch-bound/deice-ctot.csv"),
            integrated: csv("results/heathrow/branch-bound/deice-integrated.csv"),
        ),
        cplex: (
            integrated: csv("results/heathrow/cplex/deice-integrated.csv"),
        ),
    ),
)

#let objective-values(results) = results.slice(1).map(row => row.at(-2))
#let runtimes(results) = results.slice(1).map(array.last)

#let avg(..nums) = {
    let (sum, count) = nums.pos().fold((0, 0), ((sum, count), num) => (sum + num, count + 1))
    sum / count
}

#let runtime-stats(..groups) = {
    let individual = groups
        .named()
        .pairs()
        .map(pair => {
            let key = pair.first()
            let vals = pair.last()
            ((key): vals.filter(str => str.len() > 0).map(float))
        })
        .fold((:), (dict, pair) => dict + pair)

    let total = individual
        .pairs()
        .map(pair => (pair.first(), pair.last().sum()))
        .fold((:), (dict, (key, val)) => dict + ((key): val))
    
    let avg = individual
        .pairs()
        .map(pair => (pair.first(), avg(..pair.last())))
        .fold((:), (dict, (key, val)) => dict + ((key): val))
    
    (
        individual: individual,
        total: total,
        avg: avg,
    )
}

== Problem Instances

// TODO: Check if Heathrow or University of Bologna should be cited
The performance of the CPLEX model and the branch-and-bound program (utilising the three different de-icing approaches) is illustrated here using complex real-world problem instances from a single day of departure operations at London Heathrow -- whose characteristics are summarised in @table:heathrow-instances -- as well as benchmark problem instances from Milan Airport.
The latter were first introduced by #cite(<furini-improved-horizon>, form: "prose"), and were obtained from the University of Bologna Operations Research Group's freely accessible online library of instances @unibo-codes-instances.

#let heathrow-instances = results-table(
    group-headers: ([Small], [Medium], [Large]),
    csv("results/heathrow/instances-small.csv"),
    csv("results/heathrow/instances-medium.csv"),
    csv("results/heathrow/instances-large.csv"),
)

#figure(
    heathrow-instances,
    caption: [
        Overview of problem instances from London Heathrow
    ],
) <table:heathrow-instances>

The terminal maneuvering area around Heathrow is highly complex, with up to six different SID routes in use at any given time and up to five different weight classes to consider.
This results in a complex separation matrix, in which triangle inequalities are often violated -- i.e. the runway separation for an aircraft is influenced by multiple preceding aircraft rather than just the immediately preceding aircraft @demaere-pruning-rules.
Additionally, a substantial number of aircraft are also subject to CTOTs, which further reduces the number of complete orders that can be inferred.

By contrast, the Milan problem instances are significantly simpler due to having a relatively high number of separation-identical aircraft and a mix of both arrivals and departures, which allows complete orders to be inferred between a relatively large number of aircraft in each instance.

== Comparison of De-Icing Approaches <section:compare-deice>

#let convert-stat-case(stat) = (
    confidence-interval: (
        confidence-level: stat.at("confidence_interval").at("confidence_level"),
        lower-bound: stat.at("confidence_interval").at("lower_bound"),
        upper-bound: stat.at("confidence_interval").at("upper_bound"),
    ),
    point-estimate: stat.at("point_estimate"),
    standard-error: stat.at("standard_error"),
)

#let convert-stats-case(stats) = (
    mean: convert-stat-case(stats.at("mean")),
    std-dev: convert-stat-case(stats.at("std_dev")),
    median: convert-stat-case(stats.at("median")),
    median-abs-dev: convert-stat-case(stats.at("median_abs_dev")),
)

#let benches(dir, ids) = {
    ids.map(id => {
        let estimates = json(dir + str(id) + "/base/sample.json")
        let samples = estimates.iters.zip(estimates.times).map(((iters, time)) => time / iters)
        let stats = json(dir + str(id) + "/base/estimates.json")
        (id: id, samples: samples, stats: convert-stats-case(stats))
    }).fold((:), (dict, elem) => dict + ((str(elem.id)): (
        samples: elem.samples,
        stats: elem.stats,
    )))
}

#let runtime-stats(data) = {
    let runtimes = data
        .pairs()
        .map(((instance-id, bench)) => (
            instance-id,
            str(calc.round(bench.stats.mean.point-estimate / 1000000, digits: 2)),
            str(calc.round(bench.stats.std-dev.point-estimate / 1000000, digits: 2)),
            str(calc.round(bench.stats.median.point-estimate / 1000000, digits: 2)),
            str(calc.round(bench.stats.median-abs-dev.point-estimate / 1000000, digits: 2)),
        ))
    (
        ("Instance", "Mean runtime (ms)", str(sym.sigma) + " (ms)", "Median runtime (ms)", "MAD (ms)"),
        ..runtimes,
    )
}

#let avg-runtime-graph(..args) = {
    set text(size: 10pt)

    canvas({
        draw.set-style(
            axes: (stroke: 0.5pt + black),
            grid: (
                stroke: (
                    thickness: 0.5pt,
                    dash: "dotted",
                ),
            ),
            legend: (
                fill: white,
                stroke: 0.5pt + black,
                padding: 0.2,
                spacing: 0.2,
                item: (spacing: 0.2),
            ),
        )
        
        plot.plot(
            x-label: [*Instance*],
            y-label: [*Mean runtime (μs)*],
            x-format: x => [#x],
            y-format: y => [10#super[#calc.round(y)]],
            x-tick-step: 1,
            y-tick-step: 1,
            ..args.named(),
        {
            for (label, data, colour) in args.pos() {
                let instance-ids = data
                    .keys()
                    .map(int)
                let mean-runtimes = data
                    .values()
                    .map(data => data.stats.mean.point-estimate)
                    .map(rt => calc.log(rt / 1000, base: 10))
                
                let x = palette.red(0)
                plot.add(
                    instance-ids.zip(mean-runtimes),
                    label: label,
                    line: "linear",
                    style: (stroke: 1pt + colour),
                    mark: "o",
                    mark-size: 0.1,
                    mark-style: (stroke: 1pt + colour, fill: colour.lighten(50%))
                )
            }
        })
    })
}

// #let runtime-boxwhisker(..args) = {
//     set text(size: 10pt)

//     canvas({
//         draw.set-style(
//             axes: (stroke: 0.5pt + black),
//             grid: (
//                 stroke: (
//                     thickness: 0.5pt,
//                     dash: "dotted",
//                 ),
//             ),
//             boxwhisker: (
//                 mark-size: 0.1,
//             ),
//         )

//         let data = args.pos().first()
//         let boxwhisker-data = data.pairs().map(((instance-id, bench)) => {
//             let samples = bench.samples.sorted().map(n => calc.log(n, base: 10))
//             let sample-count = samples.len()

//             let median = calc.log(bench.stats.median.point-estimate, base: 10)

//             let q1-idx = (sample-count - 1) * 0.25
//             let q1-low = samples.at(calc.floor(q1-idx))
//             let q1-high = samples.at(calc.ceil(q1-idx))
//             let q1 = q1-low + (q1-high - q1-low) * calc.fract(q1-idx)

//             let q3-idx = (sample-count - 1) * 0.75
//             let q3-low = samples.at(calc.floor(q3-idx))
//             let q3-high = samples.at(calc.ceil(q3-idx))
//             let q3 = q3-low + (q3-high - q3-low) * calc.fract(q3-idx)

//             let iqr = q3 - q1

//             let min = q1 - iqr * 1.5
//             let max = q3 + iqr * 1.5

//             let low-outliers = samples.filter(n => n < min)
//             let high-outliers = samples.filter(n => n > max)
//             let outliers = low-outliers + high-outliers

//             (
//                 label: instance-id,
//                 outliers: outliers,
//                 min: min,
//                 max: max,
//                 q1: q1,
//                 q2: median,
//                 q3: q3,
//             )
//         })

//         chart.boxwhisker(
//             ..args.named(),
//             label-key: "label",
//             boxwhisker-data,
//         )
//     })
// }

@table:branch-bound-heathrow-results lists the makespans, earliest and latest de-icing times, objective values, and mean runtimes for all Heathrow problem instances solved by the branch-and-bound program utilising the three different de-icing approaches.
The small problem instances were solved without a rolling horizon, while a rolling horizon of 10 was used for the medium and large instances.
Entries for runs that fail to produce feasible solutions are left blank.

// TODO: Replace runtime here with runway hold
#let branch-bound-heathrow-results = results-table(
    group-headers: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
    side-headers: true,
    inset: (_, y) => if y <= 1 { 5pt } else { 3.5pt },
    results.heathrow.branch-bound.tobt,
    results.heathrow.branch-bound.ctot,
    results.heathrow.branch-bound.integrated,
)

#align(
    center,
    rotate(-90deg, reflow: true)[
        #figure(
            branch-bound-heathrow-results,
            caption: [
                Results for all problem instances from London Heathrow solved by the branch-and-bound program utilising the different de-icing approaches
            ],
        ) <table:branch-bound-heathrow-results>
    ],
)

#let heathrow-improvements = {
    let tobt-ctot = objective-values(results.heathrow.branch-bound.tobt)
        .zip(objective-values(results.heathrow.branch-bound.ctot))
        .filter(row => row.all(str => str.len() > 0))
        .map(row => int(row.first()) / int(row.last()))

    let tobt-integrated = objective-values(results.heathrow.branch-bound.tobt)
        .zip(objective-values(results.heathrow.branch-bound.integrated))
        .filter(row => row.all(str => str.len() > 0))
        .map(row => int(row.first()) / int(row.last()))
    
    let ctot-integrated = objective-values(results.heathrow.branch-bound.ctot)
        .zip(objective-values(results.heathrow.branch-bound.integrated))
        .filter(row => row.all(str => str.len() > 0))
        .map(row => int(row.first()) / int(row.last()))
    
    (
        tobt-ctot: avg(..tobt-ctot),
        tobt-integrated: avg(..tobt-integrated),
        ctot-integrated: avg(..ctot-integrated),
    )
}

It can be observed from @table:branch-bound-heathrow-results that the two different decomposed de-icing approaches -- by TOBT and by CTOT -- result in nearly identical makespans, earliest and latest de-icing times, and objective values across all problem instances, with decomposed de-icing by CTOT attaining only a #calc.round((heathrow-improvements.tobt-ctot - 1.0) * 100, digits: 2)% improvement in objective values on average compared to decomposed de-icing by TOBT.

However, integrated de-icing achieves #calc.round((heathrow-improvements.tobt-integrated - 1.0) * 100, digits: 2)% and #calc.round((heathrow-improvements.ctot-integrated - 1.0) * 100, digits: 2)% better objective values on average compared to decomposed de-icing by TOBT and by CTOT respoectively.
It also produces consdierably shorter makespans in the larger problem instances, indicating better runway utilisation over time compared to its decomposed counterparts -- even when using a rolling horizon.

#let branch-bound-heathrow-benches = (
    tobt: benches("benches/heathrow/deice-tobt/", range(1, 20 + 1) + range(26, 30 + 1)),
    ctot: benches("benches/heathrow/deice-ctot/", range(1, 20 + 1) + range(26, 30 + 1)),
    integrated: benches("benches/heathrow/deice-integrated/", range(1, 30 + 1)),
)

@chart:branch-bound-heathrow-avg-runtimes further shows the mean runtime for each individual problem instance solved using each de-icing approach.
It can be seen that decomposed de-icing is faster than integrated de-icing for small- and medium-sized problem instances, but slower for large instances.

#let branch-bound-heathrow-avg-runtimes = avg-runtime-graph(
    size: (12, 8),
    y-grid: true,
    y-min: 1,
    y-max: 7,
    legend: "legend.inner-north-west",
    ([Decomposed de-icing (by TOBT)], branch-bound-heathrow-benches.tobt, palette.red(3).fill),
    ([Decomposed de-icing (by CTOT)], branch-bound-heathrow-benches.ctot, palette.orange(3).fill),
    ([Integrated de-icing], branch-bound-heathrow-benches.integrated, palette.blue(3).fill),
)

#figure(
    branch-bound-heathrow-avg-runtimes,
    caption: [
        Mean runtimes for each de-icing approach of the branch-and-bound program for each problem instance from London Heathrow
    ],
) <chart:branch-bound-heathrow-avg-runtimes>

#let heathrow-total-runtimes(from, to) = (
    tobt: branch-bound-heathrow-benches
        .tobt
        .pairs()
        .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
        .map(((instance-id, bench)) => bench.stats.mean.point-estimate)
        .sum(),
    ctot: branch-bound-heathrow-benches
        .ctot
        .pairs()
        .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
        .map(((instance-id, bench)) => bench.stats.mean.point-estimate)
        .sum(),
    integrated: branch-bound-heathrow-benches
        .integrated
        .pairs()
        .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
        .map(((instance-id, bench)) => bench.stats.mean.point-estimate)
        .sum(),
)

#let heathrow-avg-runtimes(from, to) = (
    tobt: avg(
        ..branch-bound-heathrow-benches
            .tobt
            .pairs()
            .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
            .map(((instance-id, bench)) => bench.stats.mean.point-estimate),
        ),
    ctot: avg(
        ..branch-bound-heathrow-benches
            .ctot
            .pairs()
            .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
            .map(((instance-id, bench)) => bench.stats.mean.point-estimate),
        ),
    integrated: avg(
        ..branch-bound-heathrow-benches
            .integrated
            .pairs()
            .filter(((instance-id, ..)) => int(instance-id) in range(from, to + 1))
            .map(((instance-id, bench)) => bench.stats.mean.point-estimate),
        ),
)

#let heathrow-total-runtimes-all = heathrow-total-runtimes(1, 30).pairs().fold(
    (:),
    (dict, (key, total)) => dict + ((key): calc.round(total / 1000000000, digits: 2)),
)

#let heathrow-avg-runtimes-all = heathrow-avg-runtimes(1, 30).pairs().fold(
    (:),
    (dict, (key, total)) => dict + ((key): calc.round(total / 1000000, digits: 2)),
)

Even so, integrated de-icing is in fact _faster_ than its decomposed counterparts on average -- the total time taken to solve all 30 problem instances is #heathrow-total-runtimes-all.tobt seconds and #heathrow-total-runtimes-all.ctot seconds for decomposed de-icing by TOBT and by CTOT respectively, and #heathrow-total-runtimes-all.integrated seconds for integrated de-icing.
This equates to an average runtime of #heathrow-avg-runtimes-all.tobt, #heathrow-avg-runtimes-all.ctot, and #heathrow-avg-runtimes-all.integrated milliseconds respectively across all instances.
In other words, integrated de-icing is #calc.round((heathrow-avg-runtimes-all.tobt / heathrow-avg-runtimes-all.integrated - 1.0) * 100, digits: 2)% and #calc.round((heathrow-avg-runtimes-all.ctot / heathrow-avg-runtimes-all.integrated - 1.0) * 100, digits: 2)% faster on average than decomposed de-icing by TOBT and by CTOT respectively.
A breakdown of the total and average runtimes across each instance size group (small, medium, and large) is shown in @chart:heathrow-total-avg-runtimes.

// TODO: Recheck the accuracy of these numbers
Moreover, it should be noted that both decomposed approaches failed to produce feasible solutions for instances 21 through 25.
Further testing reveals that a rolling horizon of 20 or higher is required to solve these instances using decomposed de-icing; however, the resulting objective values and mean runtimes are still worse than those achieved by the integrated approach using a lower rolling horizon of 10.
As such, despite having solved less instances, both decomposed de-icing approaches have higher total (and average) runtimes than that of the integrated approach.

#let heathrow-total-avg-runtimes = {
    let totals = for (label, ..runtimes) in (
        ([Small], ..heathrow-total-runtimes(1, 10).values()),
        ([Medium], ..heathrow-total-runtimes(11, 20).values()),
        ([Large], ..heathrow-total-runtimes(21, 30).values()),
    ) {
        ((label, ..runtimes.map(rt => calc.log(rt / 1000, base: 10))),)
    }

    let avgs = for (label, ..runtimes) in (
        ([Small], ..heathrow-avg-runtimes(1, 10).values()),
        ([Medium], ..heathrow-avg-runtimes(11, 20).values()),
        ([Large], ..heathrow-avg-runtimes(21, 30).values()),
    ) {
        ((label, ..runtimes.map(rt => calc.log(rt / 1000, base: 10))),)
    }

    set text(size: 10pt)

    canvas({
        draw.set-style(
            axes: (
                stroke: 0.5pt + black,
                grid: (
                    stroke: (
                        thickness: 1pt,
                        dash: "densely-dashed",
                    ),
                ),
            ),
            legend: (
                fill: white,
                stroke: 0.5pt + black,
                padding: 0.2,
                spacing: 0.2,
                item: (spacing: 0.1),
            ),
            columnchart: (
                bar-width: 0.8,
            ),
        )

        let total-chart = chart.columnchart(
            mode: "clustered",
            size: (5, 5),
            label-key: 0,
            value-key: range(1, 4),
            labels: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
            x-label: [*Instance group*],
            y-label: [*Total runtime (μs)*],
            y-tick-step: 1,
            y-format: y => [10#super[#calc.round(y, digits: 2)]],
            bar-style: idx => (
                stroke: 0.5pt,
                fill: palette.red(idx).fill,
            ),
            legend: "legend.north",
            totals,
        )

        let avg-chart = chart.columnchart(
            mode: "clustered",
            size: (5, 5),
            label-key: 0,
            value-key: range(1, 4),
            labels: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
            x-label: [*Instance group*],
            y-label: [*Average runtime (μs)*],
            y-tick-step: 1,
            y-format: y => [10#super[#calc.round(y, digits: 2)]],
            bar-style: idx => (
                stroke: 0.5pt,
                fill: palette.indigo(idx).fill,
            ),
            legend: "legend.north",
            avgs,
        )

        draw.group(name: "total", total-chart)

        draw.group(name: "avg", anchor: "south-west", {
            draw.anchor("default", "total.south-east")
            avg-chart
        })
    })
}

#figure(
    heathrow-total-avg-runtimes,
    caption: [
        Total and average runtimes for each de-icing approach of the branch-and-bound program across each size group of problem instances from London Heathrow
    ],
) <chart:heathrow-total-avg-runtimes>

@table:branch-bound-heathrow-runtime-stats provides a more detailed overview of the runtimes of each de-icing approach for each Heathrow problem instance, including the mean runtime, standard deviation $sigma$, median runtime, and median absolute deviation (MAD).
Values that are smaller than a microsecond are displayed as zeros.

#let branch-bound-heathrow-runtimes = {
    let decomposed-missing-ids = range(21, 25 + 1)

    let tobt = runtime-stats(branch-bound-heathrow-benches.tobt)
    for instance-id in decomposed-missing-ids {
        tobt.insert(instance-id, (str(instance-id), "", "", "", ""))
    }
    
    let ctot = runtime-stats(branch-bound-heathrow-benches.ctot)
    for instance-id in decomposed-missing-ids {
        ctot.insert(instance-id, (str(instance-id), "", "", "", ""))
    }

    let integrated = runtime-stats(branch-bound-heathrow-benches.integrated)

    results-table(
        group-headers: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
        side-headers: true,
        tobt,
        ctot,
        integrated,
    )
}

#figure(
    branch-bound-heathrow-runtimes,
    caption: [Mean, standard deviation, median, and median absolute deviation of runtimes for each problem instance from London Heathrow solved by the branch-and-bound program using each de-icing approach],
) <table:branch-bound-heathrow-runtime-stats>

@table:branch-bound-furini-results lists the results for all Milan benchmark instances introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program utilising the three different de-icing approaches.
Since these instances do not contain de-icing data, the pushback duration $p_i$, pre-de-ice taxi duration $m_i$, de-icing duration $o_i$, taxi-out duration $n_i$, and lineup duration $q_i$ are assumed to be five minutes each for all departures.
Additionally, they do not consider CTOT slots, so there are no results available for decomposed de-icing by CTOT.
A rolling horizon of size 10 was used to solve each instance.
Like in @table:branch-bound-heathrow-results, entries for runs that fail to produce feasible solutions are left blank.

// TODO: Replace runtime here with runway hold
#let branch-bound-furini-results = results-table(
    group-headers: ([Decomposed de-icing], [Integrated de-icing]),
    side-headers: true,
    results.furini.branch-bound.decomposed,
    results.furini.branch-bound.integrated,
)

#figure(
    branch-bound-furini-results,
    caption: [
        Results for the Milan Airport benchmark problem instances introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program using each de-icing approach
    ],
) <table:branch-bound-furini-results>

Much like the Heathrow instances, it can be seen from @table:branch-bound-furini-results that both the decomposed and integrated approach result in very similar makespans and earliest and latest de-icing times, although integrated de-icing produces slightly longer makespans and later de-icing end times for some instances.

#let furini-integrated-improvement = {
    let obj-diffs = objective-values(results.furini.branch-bound.decomposed)
        .zip(objective-values(results.furini.branch-bound.integrated))
        .filter(row => row.all(str => str.len() > 0))
        .map(row => int(row.first()) / int(row.last()))
    avg(..obj-diffs)
}

However, the objective values achieved by the integrated de-icing approach are far better than its decomposed counterpart's -- integrated de-icing achieves a #calc.round((furini-integrated-improvement - 1.0) * 100, digits: 2)% improvement in objective values on average compared to decomposed de-icing.

#let branch-bound-furini-benches = (
    decomposed: benches("benches/furini/deice-decomposed/", range(2, 12 + 1)),
    integrated: benches("benches/furini/deice-integrated/", range(1, 12 + 1)),
)

@chart:branch-bound-furini-avg-runtimes further shows the mean runtime for each individual problem instance solved using each de-icing approach.
It can be seen that unlike the large Heathrow instances, decomposed de-icing is substantially faster than integrated de-icing for all twelve problem instances.

#let branch-bound-furini-avg-runtimes = avg-runtime-graph(
    size: (12, 8),
    y-grid: true,
    y-min: 3,
    y-max: 6,
    legend: "legend.inner-north-west",
    ([Decomposed de-icing], branch-bound-furini-benches.decomposed, palette.red(3).fill),
    ([Integrated de-icing], branch-bound-furini-benches.integrated, palette.blue(3).fill),
)

#figure(
    branch-bound-furini-avg-runtimes,
    caption: [
        Mean runtimes for each problem instance introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program using each de-icing approach
    ],
) <chart:branch-bound-furini-avg-runtimes>

#let furini-total-runtimes = (
    decomposed: branch-bound-furini-benches
        .decomposed
        .values()
        .map(bench => bench.stats.mean.point-estimate)
        .sum(),
    integrated: branch-bound-furini-benches
        .integrated
        .values()
        .map(bench => bench.stats.mean.point-estimate)
        .sum(),
)

#let furini-avg-runtimes = (
    decomposed: avg(
        ..branch-bound-furini-benches
            .decomposed
            .values()
            .map(bench => bench.stats.mean.point-estimate),
    ),
    integrated: avg(
        ..branch-bound-furini-benches
            .integrated
            .values()
            .map(bench => bench.stats.mean.point-estimate),
    ),
)

Indeed, the total time taken to solve all twelve problem instances is #calc.round(furini-total-runtimes.decomposed / 1000000, digits: 2) milliseconds and #calc.round(furini-total-runtimes.integrated / 1000000, digits: 2) milliseconds for the decomposed and integrated approaches respectively.
This equates to an average runtime of #calc.round(furini-avg-runtimes.decomposed / 1000000, digits: 2) milliseconds and #calc.round(furini-avg-runtimes.integrated / 1000000, digits: 2) milliseconds respectively, making decomposed de-icing #calc.round((furini-avg-runtimes.integrated / furini-avg-runtimes.decomposed - 1.0) * 100, digits: 2)% faster than integrated de-icing.

#let heathrow-avg-runtimes-large = heathrow-avg-runtimes(21, 30).pairs().fold(
    (:),
    (dict, (key, total)) => dict + ((key): calc.round(total / 1000000, digits: 2)),
)

In comparison, the average runtime to solve all large Heathrow problem instances -- which have the same number of aircraft as the Milan problem instances -- is #heathrow-avg-runtimes-large.tobt milliseconds, #heathrow-avg-runtimes-large.ctot milliseconds, and #heathrow-avg-runtimes-large.integrated milliseconds using decomposed de-icing by TOBT, decomposed de-icing by CTOT, and integrated de-icing respectively.

As evidenced by their much lower mean runtimes, the Milan problem instances are considerably easier to solve than the large Heathrow instances with the same number of aircraft, despite having more departures to de-ice per instance.
This is primarily due to the lack of CTOT slots as well as the presence of relatively simple separation matrices, which allows complete orders to be inferred between most aircraft in each instance.

@table:branch-bound-furini-runtime-stats provides a more detailed overview of the runtimes of each de-icing approach for each Milan airport problem instance, including the mean runtime, standard deviation $sigma$, median runtime, and median absolute deviation.

#let branch-bound-furini-runtimes = {
    let pad-instance-id(instance-id) = if instance-id == "Instance" {
        instance-id
    } else if instance-id.len() < 2 {
        "FPT0" + instance-id
    } else {
        "FPT" + instance-id
    }

    let decomposed = runtime-stats(branch-bound-furini-benches.decomposed)
        .map(((instance-id, ..rest)) => {
            (pad-instance-id(instance-id), ..rest)
        })
    decomposed.insert(1, ("FPT01", "", "", "", ""))
    
    let integrated = runtime-stats(branch-bound-furini-benches.integrated)
        .map(((instance-id, ..rest)) => {
            (pad-instance-id(instance-id), ..rest)
        })

    results-table(
        group-headers: ([Decomposed de-icing], [Integrated de-icing]),
        side-headers: true,
        decomposed,
        integrated,
    )
}

#figure(
    branch-bound-furini-runtimes,
    caption: [Mean, standard deviation, median, and median absolute deviation of runtimes for each problem instance introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program using each de-icing approach],
) <table:branch-bound-furini-runtime-stats>

// TODO: Write more about different de-icing approaches in branch-and-bound program if necessary

== Comparison of Programs <section:compare-programs>

@table:cplex-branch-bound-heathrow-results lists the makespans, earliest and latest de-icing times, and mean runtimes for all small instances from London Heathrow, solved using the mathematical program implemented in CPLEX as well as the branch-and-bound program -- both utilising an integrated de-icing approach.
The results for the latter are the same as in @table:branch-bound-heathrow-results, but are presented again here for convenience.
Both implementations achieve the same (optimal) objective values across all instances.

// TODO: Remove the objective values here
#let cplex-branch-bound-heathrow-results = results-table(
    group-headers: ([CPLEX model], [Branch-and-bound program]),
    side-headers: true,
    results.heathrow.cplex.integrated,
    results.heathrow.branch-bound.integrated.slice(0, 10 + 1),
)

#figure(
    cplex-branch-bound-heathrow-results,
    caption: [
        Results for small problem instances from London Heathrow solved by CPLEX as well as the branch-and-bound program, both utilising an integrated de-icing approach
    ],
) <table:cplex-branch-bound-heathrow-results>

#todo("Write about comparison of CPLEX model versus branch-and-bound program")

== Impact

The results discussed in @section:compare-deice clearly show that the objective values achieved by integrated de-icing are _no worse_ than those achieved by decomposed de-icing, and are often significantly better, even when a rolling horizon is involved.

#todo("Write more about impact of results")

= Reflections

#todo("Write introduction to reflection")

== Project Management

#todo("Write about project management")

// TODO: Check if a better heading could be used
== Contributions

#todo("Write about LSEPI and contributions")

= References

// NOTE: Title disabled since we want to use a custom title and passing in a heading as the title makes
//       it too big and messes up the table of contents
#bibliography("references.yml", title: none, style: "ieee")
