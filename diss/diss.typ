#import "@preview/cetz:0.2.2": canvas, chart, draw, palette, plot
#import "@preview/lovelace:0.2.0": algorithm, pseudocode-list, setup-lovelace
#import "@preview/timeliney:0.0.1": timeline

// NOTE: Needs to be called at the top of the document to setup lovelace
#show: setup-lovelace

#set text(font: "EB Garamond", size: 11pt)
#set par(justify: true)

#show outline.entry.where(level: 1): entry => {
    v(0.8em)
    strong(link(entry.element.location(), entry.body))
    h(1fr)
    strong(link(entry.element.location(), entry.page))
}

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

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

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
    #text(size: 14pt)[*Abstract*]

    #box(width: 85%, align(left)[
        Global air transportation demand has been continuously increasing in recent times -- and is only predicted to increase further, placing growing pressure on airports all around the world.
        At the same time, increasing runway capacity at airports is not easy due to the limited availability of space and the high costs of infrastructure.
        This underlines the need for efficient and intelligent scheduling of runway operations in order to minimise delays and operating costs at airports.

        Most existing approaches to runway sequencing solve the problem in isolation.
        However, approaches at integrating other optimisation problems with runway sequencing in the past have yielded significantly positive results, showing that the availability of more information can make the integrated problem easier to solve than a decomposed version.

        This dissertation thus explores the integrated runway sequencing and de-icing problem, introducing a mathematical model and a branch-and-bound algorithm for solving it to optimality.
        Furthermore, a rolling horizon heuristic is also proposed to improve the computational tractability of the latter.
        These are evaluated on a variety of problem instances from two major international airports, and compared against two different decomposed approaches to runway sequencing and de-icing.
        The results reported here indicate that integrated de-icing achieves significantly better objective values than its decomposed counterparts while still remaining well within the stringent time limits required for highly dynamic and real-time systems.
    ])
]

#v(1fr)

#pagebreak()

#outline(indent: auto)
#pagebreak()

// NOTE: Done after cover page, abstract, and table of contents since we don't want page numbers to show up on them
#set page(numbering: "1")

// NOTE: Forces the page numbering to begin from here rather than from the cover page or abstract page
#counter(page).update(1)

= Introduction <section:introduction>

An airport's maximum capacity and throughput -- i.e. the number of aircraft landing or taking off per unit of time -- is bounded by the capacity of its runway(s) @lieder-dynamic-programming.
This effecively makes an airport's runway systems a bottleneck in the global air traffic network @lieder-scheduling-aircraft.
However, there is an ever-increasing demand for air transportation @lieder-scheduling-aircraft @demaere-pruning-rules @furini-improved-horizon, increasing the pressure on already limited airport resources.
The total runway capacity of an airport can be increased by adding more runways -- however, this is often not feasible due to the high infrastructure costs and long-term planning required, nor is it always possible due to the limited availability of land @demaere-pruning-rules.
Therefore, the efficient use of existing runways by intelligently scheduling runway operations -- i.e. landings and take-offs -- is crucial for maximising the capacity of airports and minimise delays, fuel emissions, and operating costs.

The runway sequencing problem refers to the NP-hard problem @demaere-pruning-rules of finding a feasible sequence of landing and take-off times for a given set of aircraft such that a set of constraints is satisfied and an optimal value for a given objective function is met.
Aircraft landing on or taking off from a given airport must comply with strict separation requirements that are dictated by the aircraft classes of the preceding and succeeding operations, among other factors @lieder-scheduling-aircraft @lieder-dynamic-programming.
Aircraft may also be subject to a number of other constraints, including earliest landing or take-off times, hard time windows during which they must land or take off, and Calculated Take-Off Time (CTOT) slots (for departures).
Moreover, aircraft that are taking off may also require application of de-icing fluid to prevent the formation of frost or ice on their surface, which could otherwise compromise their aerodynamic stability.
This imposes further constraints on the take-off times of these aircraft since de-icing fluid only remains effective for a certain period of time -- called the Holdover Time (HOT) -- during which the aircraft must take off.
Such constraints -- which are further detailed in @section:problem-description -- affect the order on which aircraft land or take-off, which in turn affects the delays for each aircraft as well as the overall runway utilisation.

Prior approaches to runway sequencing have mainly focused on the problem in isolation or consider runway sequencing and de-icing in a decomposed or partially integrated manner -- i.e. generating solutions for the two problems independently of each other and then combining them.
Many of these approaches are discussed in @section:literature.
However, solutions that integrate runway sequencing with other problems -- such as the integrated runway sequencing and ground movement problem explored by #cite(<atkin-hybrid-metaheuristics>, form: "prose") -- have shown that the availability of more information from their integration can offer key insights into simplifying the problem(s) and produce better solutions overall.
It is therefore important to investigate the feasibility of an integrated approach to runway sequencing and de-icing and evaluate its effectiveness by comparing it to fully decomposed or partially integrated approaches.

This project thus introduces a mathematical model for the integrated runway sequencing and de-icing problem and a branch-and-bound algorithm for solving it.
In doing so, this project provides fundamental insights into the characteristics of runway sequencing and de-icing and the advantages of integrated de-icing over decomposed de-icing, which is of value to future research in the field.
This dissertation is structured as follows:

@section:literature contains an overview of the existing literature on runway sequencing as well as some of the past approaches that are (or are not) adopted in this project.
An in-depth description of the integrated runway sequencing and de-icing problem is provided in @section:problem-description, with @section:constraints and @section:objectives further providing detailed explanations of the constraints and objectives respectively.
A mathematical model for the problem is introduced in @section:model.

@section:implementation then presents the two implementations of the aforementioned model -- a mathematical program and a branch-and-bound algorithm, both capable of solving the problem to optimality.
These implementations are discussed individually in @section:mathematical-program and @section:branch-bound respectively.
Additionally, the branch-and-bound program also incorporates two decomposed approaches to runway sequencing and de-icing -- each using a different method for determining the order in which aircraft de-ice.
These decomposed approaches are discussed further in @section:deice-decomposed, while the calculation of landing or take-off times and de-icing times using the integrated approach is explained in @section:deice-integrated.

@section:results presents the results of evaluating both the mathematical program as well as the branch-and-bound algorithm on multiple real-world problem instances obtained from two major airports in Europe -- namely London Heathrow and Milan Linate.
The integrated approach and both decomposed approaches implemented by the branch-and-bound algorithm are compared on the basis of their resulting objective values, runway utilisation, and runway hold times, as well as their runtimes.
The mathematical program and the branch-and-bound program -- both using integrated de-icing -- are also compared using a subset of the problem instances from London Heathrow.
The impact of these results is discussed in @section:impact.

Finally, @section:reflections reflects on the management and progress of this project, its broader implications as a whole concerning Legal, Social, Ethical, and Professional Issues (LSEPI), and its potential future directions.
@section:conclusion concludes this dissertation, re-iterating the impact of the results discussed in @section:results and the conclusions drawn from them.

= Existing Literature <section:literature>

Early approaches to runway sequencing used by many airports around the world include simple first-come, first-served (FCFS) algorithms optimising for a single objective @bianco-minimizing-time.
Although very simple to implement and computationally inexpensive, FCFS strategies are well-known to produce excessive delays @bianco-minimizing-time.
Therefore, a number of approaches that optimise certain aspects -- such as delay, runway utilisation, and CTOT compliance -- without violating safety constraints or compromising the tractability of the problem have been proposed in the past.
These approaches can be grouped into two main categories -- exact and approximate methods, which are discussed in detail in @section:exact-methods and @section:approximate-methods respectively.

== Exact Methods <section:exact-methods>

Exact methods produce a global optimal solution (if possible) to a problem.
Many exact methods have been introduce in the literature on runway sequencing, including mathematical programming, dynamic programming (DP), and branch-and-bound, which are discussed in detail in the following sections.

=== Mathematical Programming

Two of the most common mathematical programming techniques for solving runway sequencing (and more generally, machine scheduling) are linear programming (LP) and mixed-integer programming (MIP).
LP models represent their constraints and objectives using linear relationships.
By contrast, MIP formulations have one or more discrete variables -- i.e. they are constrained to be integers or binary values.
The use of discrete variables expands the scope of the problems that can be formulated using MIP, since certain values or constraints may not be linearisable -- however, such models are typically more difficult to solve than LP models.

#cite(<beasley-scheduling-aircraft>, form: "prose") introduce a LP-based tree search approach for the problem of sequencing arrivals on a single runway, and later extend their formulation to handle multiple runways as well.
They consider hard time window constraints and use an objective function that penalises landing before or after a given target time for each arrival.
Unlike many previous approaches that assumed an indefinite latest time limit for landing, their approach employs more realistic latest landing times based on fuel considerations.
This allows for simplifying the problem by exploiting the presence of increased disjoint intervals, caused by relatively narrow hard time windows.

Many existing MIP approaches to runway sequencing view the problem as a variant of classical machine scheduling problem with sequence-dependent setup times -- the runways correspond to machines, flights to jobs, and runway separations to setup times @avella-time-indexed.
According to #cite(<avella-time-indexed>, form: "prose"), the literature on machine scheduling has traditionally comprised of two main kinds of formulations -- big-$M$ and time-indexed.

Big-$M$ formulations represent the landing or take-off time of an aircraft as a single continuous variable.
However, such formulations typically need to resort to some heuristic approaches -- such as rolling horizons -- on top of the core formulation itself in order to meet computation time limits for instances of practical interest @avella-time-indexed.

In contrast to big-$M$ formulations, time-indexed formulations discretise the overall time horizon in small time periods.
The schedule of an aircraft is modelled by a set of binary decision variables, only one of which will be 1 in any feasible solution -- the one identifying the aircraft's landing or take-off time.
In general, time-indexed formulations return much stronger bounds than big-$M$ formulations, but at the cost of increasing the number of variables and constants, and consequently computation times @avella-time-indexed.
This makes them often unattractive for real-time applications of runway sequencing, where there are stringent limits on the computation times @bennell-runway-scheduling.

#cite(<avella-time-indexed>, form: "prose") counter this claim by presenting a time-indexed MIP formulation based on a novel class of valid clique inequalities for the single machine scheduling problem with sequence-dependent setup times.
They generalise a family of inequalities introduced by #cite(<nogueira-mixed-integer>, form: "prose").
Their formulation significantly improve the quality of the lower bounds, reduces the number of constraints, and is capable of solving difficult real-world instances from large airports in Europe, namely Stockholm Arlanda, Hamburg, and Milan Linate.

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

== Approximate Methods <section:approximate-methods>

In contrast to exact methods, approximate methods utilise techniques such as heuristics to find a solution without necessarily finding a global optimum -- although they will often find a near-optimal solution.
However, approximate methods can often find solutions very quickly -- typically faster than most exact methods.
This makes them a better choice for solving large problem instances which may not be solvable within the required time limit, as is often the case with real-time applications.

#cite(<bianco-minimizing-time>, form: "prose") propose two heuristic approaches -- Cheapest Addition Heuristic (CAH) and Cheapest Insertion Heuristic (CIH) -- that each generate sequences by either appending remaining aircraft to partial sequences, or inserting them.
They note that the latter almost always performed better than the former, as it searches for the best partial sequences obtained by inserting new aircraft anywhere within the sequence as opposed to just at the end.
However, it is also much more computationally expensive -- these heuristics are shown to have computational complexities of $O(n^2 log(n))$ and $O(n^4)$ respectively @bianco-minimizing-time @bennell-runway-scheduling.
The heuristics are evaluated on mainly randomly generated test instances.

#cite(<atkin-hybrid-metaheuristics>, form: "prose") introduce a hybridised approach using different search methods and metaheuristics to solve the integrated runway scheduling and ground movement problem at London Heathrow.
They consider total delay, CTOT compliance, positional delay, and throughput as part of their objective function.
Their results show that the availability of more information about aircraft taxiing from the integration of the problem can reduce delays and CTOT violations.

Approximate methods may also be combined with exact methods -- such as by using a heuristic to break down a large problem instance into smaller parts that can each be solved to optimality, or by providing a near-optimal solution as the initial solution to an LP or MIP solver, which allows it to find an optimal solution quicker.
This makes the overall method heuristic.

#cite(<furini-improved-horizon>, form: "prose") explore such a hybrid approach by introducing a rolling horizon approach to partition a set of aircraft into chunks and solves the runway sequenching problem to optimality for each of these chunks.
The resulting partial solutions are then re-combined into a complete solution for the original set of aircraft.
They compare three different approaches to generating such chunks of aircraft, and show that the combination of multiple chunking rules leads to an improvement with respect to an approach that does not utilise chunking.
A similar rolling horizon algorithm for the branch-and-bound algorithm introduced in this dissertation is also presented in @section:rolling-horizon.

== Paradigms

Approaches to runway sequencing can also be grouped into two different categories or paradigms -- constrained position shifts (CPS) and pruning rules.
These aim to reduce the complexity of the problem and improve the computational tractability of solutions, so that they may be of practical use in scenarios with harsh time limits, such as in real-time applications.

=== Constrained Position Shifts

A number of approaches in the past -- such as that of #cite(<psaraftis-dynamic-programming>, form: "prose") and #cite(<balakrishnan-runway-operations>, form: "prose") -- have employed CPS, which was first introduced by #cite(<dear-dynamic-scheduling>, form: "prose").
CPS restricts an aircraft's maximum shift in position relative to its original position in some initial sequence, which is typically obtained using a FCFS approach.
Not only does this prune the search space by reducing the number of aircraft that must be considered for each position in the sequence, but it also enforces positional equity by preventing aircraft from being advanced or delayed disproportionately compared to other aircraft @dear-dynamic-scheduling @demaere-pruning-rules.

According to #cite(<demaere-pruning-rules>, form: "prose"), although CPS can be an effective and efficient approach in many cases of arrival sequencing, it is overall impractical in mixed-mode operations due to the large differences in delay between arrivals and departures.

#cite(<atkin-tsat-allocation>, form: "prose") further show that when CTOT slots are considered, CTOT compliance and positional equity are heavily in conflict -- there is a tradeoff between the number of CTOT violations and positional equity.
Moreover, having a hard constraint of or high penalty for positional equity may be highly counter-productive for take-offs even apart from its conflict with delay or CTOT compliance @atkin-tsat-allocation.

For instance, there may be an aircraft that must wait for the start of its CTOT slot, during which other aircraft may be sequenced with no additional delay -- however, penalising positional inequity would (wrongfully) penalise such a sequence, forcing the other aircraft to take off after the one with the CTOT slot and increasing the overall delay in the process @atkin-tsat-allocation.

The differing delays that accumulate across different Standard Instrument Departure (SID) routes, hard time window constraints, and CTOT constraints can thus require large maximum position shifts to obtain good runway sequences, thereby challenging the tractability of CPS-based approaches @demaere-pruning-rules.
The model and branch-and-bound program presented in @section:model and @section:branch-bound of this dissertation therefore do not employ CPS, making them more practical and viable for real-world scenarios considering departures with complex separation requirements and CTOT compliance.

=== Pruning Rules

In contrast to approaches like CPS that reduce the search space of the problem by limiting the positional shifting of aircraft within the sequence, pruning rules exploit the characteristics of the problem or objective function to infer that a current sequence (or any future sequences based on it) is sub-optimal.
This has the advantage of being able to prune partial subsequences that show known poor characteristics much earlier @demaere-pruning-rules.

Pruning rules have been extensively studied in the literature involving machine scheduling @demaere-pruning-rules @allahverdi-survey-scheduling.
However, #cite(<allahverdi-survey-scheduling>, form: "prose") show that a majority of these approaches do not consider sequence-dependent setup times, despite them being prevalent in runway sequencing problems and in many other applications of machine scheduling.
Nor do they consider complex non-linear, non-convex, or discontinuous objective functions @demaere-pruning-rules.

#cite(<demaere-pruning-rules>, form: "prose") introduce a set of pruning principles that exploit the inherent characteristics of the runway sequencing problem including complete orders, conditional orders, insertion dominance, and dominance with lower bounding.
Their pruning rules enable significant reductions of the problem's computational complexity without compromising the optimality of the generated solutions, and are usually much more computationally efficient compared to pruning rules based on local improvements @demaere-pruning-rules.

Furthermore, they show that many of the pruning rules considered transfer to other objective functions commonly considered in the literature, and can thus be used outside of the specific DP approach developed by them @demaere-pruning-rules.
A subset of these pruning rules is thus incorporated into the model presented in this dissertation to improve its tractability.

= Problem Description <section:problem-description>

Given a set of arrivals $A$ and departures $D$, the runway sequencing and de-icing problem for a single runway and single de-icing station consists of finding a sequence of landing and take-off times as well as a sequence of de-icing times such that an optimal value is achieved for a given objective function, subject to the satisfaction of all hard constraints.

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
    $x_i$, [Earliest possible landing or take-off time for aircraft $i$],
    $b_i$, [Base time for aircraft $i$, used in calculating delay],
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

== Constraints <section:constraints>

A feasible solution to the problem must satisfy precedence constraints, separation requirements, earliest landing or take-off times, hard time windows, CTOT slots, holdover times, and runway hold times.
A sequence that violates these hard constraints is considered to be infeasible, and can thus be eliminated from the solution space.

=== Precedences

Since this is a single runway formulation, no two aircraft can land or take off at the same time.
Let $gamma_(i, j)$ be a boolean decision variable indicating whether aircraft $i$ lands or takes off before aircraft $j$.
The following constraint can then be imposed on every pair of distinct aircraft $(i, j)$:

$ gamma_(i, j) + gamma_(j, i) = 1 $

That is, either $i$ must land or take off before $j$ or vice-versa.
Similar precedence constraints exist for de-icing -- given any two distinct departures $i$ and $j$, either $i$ must finish its de-icing before $j$ can start de-icing, $j$ must finish its de-icing before $i$ can start:

$ z_j >= z_i + o_i or z_i >= z_j + o_j $

=== Runway Separations

Any two consecutive aircraft $i$ and $j$ (where $i$ precedes $j$) are required to have a minimum _runway separation_ $delta_(i, j)$ between them, which is determined by their weight classes in the case of arrivals, or their weight classes, speed groups, and Standard Instrument Departure (SID) routes in the case of departures.

An aircraft's weight class influences the severity of wake turbulence it produces during flight, the time required for this turbulence to dissipate, and its sensitivity to the wake turbulence caused by other aircraft.
Larger or heavier aircraft typically generate greater turbulence, to which smaller or lighter aircraft are more sensitive.
As such, a greater separation may be required when a large or heavy aircraft is followed by a small or light one, than when a small or light aircraft is followed by a large or heavy one @demaere-pruning-rules.

Similarly, a larger separation may be required when a slow aircraft is followed by a faster one on the same SID route, to prevent the latter from catching up to the former before their routes diverge.
The climb and relative bearing of the route also influence separation requirements for aircraft.
Additionally, congestion in downstream airspace sectors also has an impact on separation requirements -- in some cases, the separation between two consecutive aircraft may need to be increased to space out traffic and prevent en-route sectors and controllers from being overwhelmed @demaere-pruning-rules.

The minimum separation that must be maintained between two aircraft is thus the maximum of the separations due to their weight classes, speed groups, and SID routes.
The required separations between each ordered pair of distinct aircraft can therefore be expressed as a separation matrix @demaere-pruning-rules.

However, runway separations do not necessarily obey the _triangle inequality_ -- i.e. for any three aircraft $i$, $j$, and $k$, the inequality $delta_(i, j) + delta_(j, k) >= delta_(i, k)$ is not necessarily true @demaere-pruning-rules.
An aircraft's landing or take-off time can thus be influenced by not just the immediately preceding aircraft, but by multiple preceding aircraft.

=== Earliest Times

Every aircraft $i$ has an earliest possible time $x_i$ it can land or take off.
This is modelled as a hard constraint -- i.e. $i$ cannot be scheduled to land or take off before $x_i$:

$ t_i >= x_i $

=== Time Windows

If an aircraft $i$ is subject to a hard time window which is defined by its earliest (start) time $e_i$ and latest (end) time $l_i$, then its landing or take-off time $t_i$ must be within this window:

$ e_i <= t_i <= l_i $

In this model, each aircraft is assumed to be subject to a hard time window, although this is not always the case in the real world.
However, this assumption can be made without loss of generality -- an aircraft $i$ that is not subject to a hard time window can instead be considered to be subject to a very large time window, such that its start time $e_i$ is early enough and its end time $l_i$ late enough so as to never affect solutions in practice @demaere-pruning-rules.

=== Calculated Take-Off Times

In addition to a hard time window, a departure $i$ might be subject to a Calculated Take-Off Time (CTOT) slot, during which it should take off.
Typically, a CTOT has a tolerance of -5 to +10 minutes -- i.e. five minutes before and ten minutes after $c_i$ -- and its time window can thus be defined by its earliest (start) time $u_i$ and latest (end) time $v_i$; however, this model allows for customizable CTOT tolerances per departure.

Much like a hard time window, a departure cannot take off before $u_i$, but it may be scheduled after $v_i$ -- although this is heavily penalised.
The start time of a CTOT slot is thus modelled as a hard constraint, while its end time is modelled as a soft constraint:

$ t_i >= u_i $

=== Holdover Times

Once a departure $i$ has been de-iced, the applied de-icing fluid will remain effective for a certain duration of time, called the Holdover Time (HOT) $h_i$.
Departures must take off within this period of time -- if a departure's HOT expires before it takes off, it must be de-iced again, which could extend the de-icing queue and delay subsequent aircraft.

The HOT of a departure $i$ is thus modelled as a hard constraint -- the time between its de-ice time $z_i$ and take-off time $t_i$ must not be greater than $h_i$:

$ t_i - z_i - o_i <= h_i $

=== Runway Hold Times

Delays are ideally absorbed by stand holding -- a departure $i$ only needs to push back only when absolutely necessary to meet its de-ice time $z_i$ (if applicable) and take-off time $t_i$.

However, in some cases it may be better to absorb delays at the runway by _runway holding_ instead -- i.e. arriving and waiting at the runway before a departure's scheduled take-off time.
A departure that pushes back earlier than absoltuely necessary would be able to de-ice earlier than necessary, freeing up the de-icing queue earlier.
This could in turn enable the following departures to de-ice earlier and potentially reduce the total delay and CTOT violations in the remaining sequence.

The maximum runway holding duration $w_i$ for a departure $i$ is thus modelled as a hard constraint -- the time between $z_i$ and $t_i$ must not be greater than the sum of its de-ice duration $o_i$, post de-ice taxi duration $n_i$, lineup duration $q_i$, and maximum runway holding duration $w_i$:

$ t_i - z_i <= o_i + n_i + q_i + w_i $

== Objectives <section:objectives>

The objective function $f(s)$ for a partial or final sequence $s$ is defined in @eq:objective-function.
It considers total delay and CTOT compliance, and is based on the function described by #cite(<demaere-pruning-rules>, form: "prose").

=== Runway Utilisation

The runway utilisation of a partial or final sequence $s$ is modelled as the _makespan_ of $s$, --i.e. $max_(i in s) t_i$.
Although not directly included as an objective, it is utilised for the evaluation of partial sequences generated by the branch-and-bound program and their subsequent pruning according to the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose").

=== Delay <section:delay>

The delay for an aircraft $i$ is defined as the difference between its landing or take-off time $t_i$ and its _base time_ $b_i$ -- the latter of which is defined as the time the aircraft enters the local airspace (for arrivals) or as the time the aircraft enters the runway queue and finishes lining up (for departures).
The aircraft's delay cost $c_d (i)$ -- defined in @eq:delay-cost -- is then calculated as the delay squared, and is equivalent to the following function:

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

A time-indexed formulation is employed in order to linearise the objective function and hence solve the integrated runway sequencing and de-icing problem using 0-1 integer linear programming.

First, the landing or take-off time of an aircraft $i$ is constrained to lie between the earliest possible time $i$ can be scheduled to land or take off -- its _release time_ $r_i$ and the latest possible time $i$ can be scheduled to land or take off -- its _due time_ $d_i$.
The release time of $i$ be calculated as the maximum of its earliest time $x_i$, base time $b_i$, start time $e_i$ of its hard time window, and start time $u_i$ of its CTOT slot (if applicable):

$ r_i = max(x_i, b_i, e_i, u_i) $

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

Putting together these constraints and objectives, a 0-1 integer linear model for the integrated runway sequencing and de-icing problem is presented below:

#multi-equation[
    $ "Minimise" space &f(s) = sum_(i in s) (c_d (i) + c_v (i)) $ <eq:objective-function>
    $ &c_d (i) = sum_(t in T_i) (tau_(i, t) dot (t - b_i)^2) &forall i in F $ <eq:delay-cost>
    $ &c_v (i) = sum_(t in T_i) (tau_(i, t) dot (t > v_i) dot (t - v_i)^2) &forall i in D $ <eq:ctot-violation-cost>
    $ &t_i = sum_(t in T_i) (tau_(i, t) dot t) &forall i in F $ <eq:scheduled-time>
    $ &z_i = sum_(z in Z_i) (zeta_(i, z) dot z) &forall i in D $ <eq:deice-time>
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

=== Disjoint Time Windows <section:disjoint-time-windows>

#cite(<beasley-scheduling-aircraft>, form: "prose") show that it can be determined for certain pairs of distinct aircraft $(i, j)$ whether $i$ lands or takes off before $j$ does, based on their sets of possible landing or take-off times.
For example, if two aircraft $i$ and $j$ have their release times and due times as $r_i = 10$, $d_i = 50$, $r_j = 70$, and $d_j = 110$ respectively, then it is clear that $i$ must land or take off first -- i.e. before $j$ -- since $T_i$ and $T_j$ are disjoint.
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

=== Complete Orders <section:complete-orders>

A _complete order_ exists between any two aircraft $i$ and $j$ if the objective value of a sequence $s$ containing both $i$ and $j$ cannot be improved any further by reversing the order of $i$ and $j$ in $s$.
Exploiting complete orders by creating ordered sets of aircraft simplifies the problem of runway sequencing (or more generally, machine scheduling) to one of interleaving these ordered sets, always sequencing only the first available aircraft from each set @demaere-pruning-rules.
This reduces the problem's worst-case computational complexity from $O(n!)$ to $O(m^2 (n + 1)^m)$, where $n$ denotes the number of aircraft, and $m$ denotes the number of distinct aircraft types @psaraftis-dynamic-programming.

#cite(<psaraftis-dynamic-programming>, form: "prose") first showed the existence of such complete orders between _separation-identical_ aircraft.
Two distinct aircraft $i$ and $j$ are separation-identical if their mutual separations with respect to every other aircraft $k in F$ are the same -- i.e. $i$ and $j$ are separation-identical if and only if:

$ forall k in F, k != i and k != j and delta_(i, k) = delta_(j, k) and delta_(k, i) = delta_(k, j) $ <eq:are-separation-identical>

Additionally, #cite(<demaere-pruning-rules>, form: "prose") show that a complete order may be inferred upon a set of separation-identical aircraft if the complete orders for each of the individual objectives are consistent within the set -- i.e. as long as there is a consistent order between every aircraft's base times, release times, and hard time windows.
A complete order can thus be inferred between two separation-identical aircraft $i$ and $j$ if and only if:

$ b_i <= b_j and r_i <= r_j and l_i <= l_j $ <eq:are-complete-ordered>

However, complete orders cannot be inferred between two separation-identical aircraft if one or both aircraft are subject to CTOT slots, due to the piecewise, discontinuous, and non-convex nature of the CTOT violation cost function $c_v (i)$ @demaere-pruning-rules.
Thus, in addition to satisfying @eq:are-complete-ordered, neither of the two aircraft must be subject to a CTOT slot.

Following from @eq:are-separation-identical and @eq:are-complete-ordered, it is possible to define the set $F_C$ of pairs of distinct aircraft $(i, j)$ where $i$ and $j$ are separation-identical and have a complete order such that $i$ must land or take off before $j$:

$ F_C = { (i, j) | &(forall k in F, k != i and k != j and delta_(i, k) = delta_(j, k) and delta_(k, i) = delta_(k, j))\
    &and b_i <= b_j and r_i <= r_j and l_i <= l_j, i in F, j in F, i != j } $

The following precedence and separation constraints can thus be imposed on every pair of aircraft $(i, j) in F_C$, corresponding to @constraint:certain-precedence and @constraint:certain-separation in the model:

$ gamma_(i, j) = 1 $
$ t_j >= t_i + delta_(i, j) $

= Implementation <section:implementation>

Implementations of the aforementioned model as well as a branch-and-bound algorithm are discussed in @section:mathematical-program and @section:branch-bound respectively, with a rolling horizon extension to the latter presented in @section:rolling-horizon.
Additionally, a tool for visualising the runway sequences generated by these algorithms has also been developed -- this is discussed in @section:sequence-vis.

== Mathematical Program <section:mathematical-program>

// TODO: Write more about CPLEX implementation if necessary
The model presented in in @section:model has been implemented as a mathematical program in Optimisation Programming Language (OPL) @opl, which is packaged with IBM's ILOG CPLEX Optimisation Studio @cplex.

== Branch-and-Bound Program <section:branch-bound>

// TODO: Explain why Rust is used if necessary
A branch-and-bound algorithm to optimally solve the integrated runway sequencing and de-icing problem as described in @section:model (as well as its decomposed version) is also developed, using a depth-first-search that incrementally builds up sequences by adding one aircraft to the current partial sequence at every step.
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
    - *input* set of aircraft $F$
    - *output* optimal sequence of landings, take-offs, and de-icing times
    + $c <- 0$
    + $c_"best" = infinity$
    + $s <- $ empty sequence
    + $s_"best" <- s$
    + $Q <- $ empty stack
    + *for* each $X$ *in* ordered sets of separation-identical aircraft *do*
        + $i <- $ first aircraft in $X$
        + schedule $t_i$ and $z_i$
        + push $(i, 0)$ to $Q$
    + *end*
    + *while* $Q$ is not empty *do*
        + $(i, k) <- $ pop from $Q$
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
                        + push $(j, k + 1)$ to $Q$
                    + *end*
                + *end*
            + *end*
        + *else*
            + reset $t_i$ and $z_i$
        + *end*
    + *end*
    + *return* $s_"best"$
]

#algorithm(
    branch-bound-code,
    caption: [
        Branch-and-bound for runway sequencing and de-icing
    ],
) <code:branch-bound>

The algorithm presented above is de-icing approach-agnostic -- i.e. it only describes the branch-and-bound procedure itself, not how an individual aircraft is assigned a landing or take-off time or a de-icing time.
The benefit of this is twofold -- first, it allows for generic and easily extensible implementations of de-icing approaches, and second, it allows each de-icing approach to be compared as fairly as possible since the only difference in code comes from scheduling individual aircraft.

The two different de-icing approaches -- decomposed and integrated, wherein the former is further split into decomposed by TOBT and decomposed by CTOT -- are discussed further in the sections below.

=== Decomposed De-Icing <section:deice-decomposed>

Under decomposed de-icing, a de-icing queue is first generated before performing the main branch-and-bound procedure.
This queue is then used to fix the de-icing times of departures before scheduling their take-off times.

To generate the de-icing queue, a list of all departures to be de-iced is first created and then sorted by either their TOBTs or their CTOT slots, depending on which decomposed de-icing approach is to be used.
In the latter case, additional sorting may be necessary to prioritise aircraft with CTOT slots over those without and ensure maximal CTOT compliance.
Aircraft in this list may then be assigned de-icing times in a FCFS manner, taking into account their release times and the time that the previous aircraft finishes de-icing.
This process is shown in @code:deice-queue.

#let deice-queue = pseudocode-list[
    - *input* set of departures to de-ice $D$
    - *output* de-icing times for all departures in $D$
    + $Q <- $ empty queue
    + $z_"prev" <- $ none
    + sort $D$ by TOBT or by CTOT
    + *for* $i$ *in* $D$ *do*
        + *if* $i$ must de-ice *then*
            + $z_i <- max(r_i - q_i - n_i - o_i, r_i - h_i - o_i)$
            + *if* $z_"prev" = $ none *then*
                + $z_i <- max(z_i, z_"prev")$
            + *end*
            + $z_"prev" <- z_i$
            + push $z_i$ to $Q$
        + *end*
    + *end*
    + *return* $Q$
]

#algorithm(
    deice-queue,
    caption: [
        Generating de-icing queues in decomposed de-icing
    ],
) <code:deice-queue>

The de-icing time $z_i$ of a departure $i$ within the de-icing queue $Q$ is thus given by the following expression, where $Q_i$ is the set of departures that have been scheduled to de-ice before $i$:

$ z_i = max(r_i - q_i - n_i - o_i, r_i - h_i - o_i, max_(j in Q_i) z_j + o_j) $ <eq:deice-queue>

Following from @eq:deice-queue, the take-off time $t_i$ of a departure $i$ that has already been scheduled to de-ice can be calculated as the maximum of its release time $r_i$, de-icing time $z_i$ plus the time taken to taxi to the runway after de-icing, and $t_j + delta_(j, i)$ for every $j in s_i$, where $s_i$ is the partial sequence of all aircraft that have been sequenced before $i$:

$ t_i = max(r_i, z_i + o_i + n_i + q_i, max_(j in s_i) t_j + delta_(j, i)) $

On the other hand, if $i$ is an aircraft which is either an arrival or a departure that is not required to de-ice, its landing or take-off time does not need to consider the second component, and can simply be given by:

$ t_i = max(r_i, max_(j in s_i) t_j + delta_(j, i)) $ <eq:earliest-landing>

=== Integrated De-icing <section:deice-integrated>

Unlike its decomposed counterpart, integrated de-icing does not calculate a de-icing queue beforehand, but instead calculates the de-icing times for aircraft along with their landing or take-off times.

If an aircraft $i$ is an arrival or a departure that is not required to de-ice, then its landing or take-off time $t_i$ can be calculated as given by @eq:earliest-landing.
However, if $i$ is a departure and is required to de-ice before taking off, then its earliest de-icing time must also be taken into consideration.
This can be calculated as the time that the preceding aircraft in the de-icing queue finishes de-icing, plus the departure $i$'s de-icing duration $o_i$, taxi-out duration $n_i$, and lineup duration $q_i$.
Thus, the take-off time $t_i$ for a departure $i$ that must de-ice is given by:

$ t_i = max(r_i, max_(j in s_i) t_j + delta_(j, i), max_(k in s_i) z_k + o_i + n_i + q_i) $ <eq:earliest-takeoff>

Once its take-off time is known, its de-icing time $z_i$ can be calculated as the maximum of its earliest de-icing time (given above in @eq:earliest-takeoff), $t_i$ minus the sum of its HOT $h_i$ and de-icing duration $o_i$, and $t_i$ minus the sum of its maximum allowed runway hold duration $r_i$, lineup duration $q_i$, taxi-out duration $n_i$, and de-icing duration $o_i$:

$ z_i = max(t_i - q_i - r_i - n_i - o_i, t_i - h_i - o_i, max_(j in s_i) z_j + o_i + n_i + q_i) $

=== Rolling Horizon Extension <section:rolling-horizon>

Altough the branch-and-bound algorithm shown in @code:branch-bound produces optimal solutions, it scales poorly with the size of the input.
When attempting to optimally solve very challenging problem instances with any more than 15 aircraft -- such as those considered in the first half of @section:compare-deice -- the runtime of the algorithm quickly surpasses the stringent time limits required by real-time applications.

A rolling horizon extension is thus presented to counter this and improve the tractability of the branch-and-bound algorithm.
The main idea of a rolling horizon is to consider subsets of a large set of aircraft, which are small enough to be solved in real-time, and to then reconstruct a full solution to the set of aircraft from the partial solutions of each subset @furini-improved-horizon.
It reduces the computational cost for runway sequencing, albeit at the expense of optimality -- it is a heuristic method that is not guaranteed to produce an optimal solution.

The rolling horizon algorithm works by repeating the branch-and-bound search $(|F| - k)$ times -- where $k$ is the size of the rolling horizon -- and saving only the first sequenced aircraft from each iteration to the current sequence $s$.
The remaining aircraft that are not selected from the optimal solution for each iteration are available to be sequenced again the following iteration.
In the final iteration, all aircraft from the best solution produced by the branch-and-bound algorithm are appended to the current sequence $s$, instead of just the first aircraft.
The full rolling horizon extension is shown below.

#let rolling-horizon = pseudocode-list[
    - *input* set of aircraft $F$, size of rolling horizon $k$
    - *output* sequence of landings, take-offs, and de-icing times
    + $s_"best" <- $ empty sequence
    + $s_"prev" <- s_"best"$
    + $k_"start" <- 0$
    + $k_"end" <- min(k, |F|)$
    + $s_"prev" <- $ perform branch-and-bound up to $k_"end"$ aircraft
    + *while* $k_"end" < |F|$ *do*
        + $i <- $ first aircraft in $s_"prev"$
        + push $i$ to $s$
        + remove $i$ from ordered sets of separation-identical aircraft
        + $k_"start" <- k_"start" + 1$
        + $k_"end" <- k_"end" + 1$
        + $s_"prev" <- $ perform branch-and-bound up to $k_"end"$ aircraft
    + *end*
    + append $s_"prev"$ to $s_"best"$
    + *return* $s_"best"$
]

#show figure.where(kind: "lovelace"): set box(width: 1fr)

#algorithm(
    rolling-horizon,
    caption: [
        Rolling horizon for solving larger problem instances using the branch-and-bound program
    ],
) <code:rolling-horizon>

== Sequence Visualiser <section:sequence-vis>

The visualiser takes any sequence of landing or take-off times and de-icing times, and produces a Scalable Vector Graphic (SVG) file showing.
SVG was chosen as the image format due to wide support for SVG rendering in many browsers and image applications, and because its XML-like syntax makes SVG files relatively easy to create and manipulate within code.

#figure(
    image("15.svg", width: 100%, alt: "Visualiser output for an instance with 35 aircraft"),
    caption: [
        Visualiser output for an instance with 35 aircraft
    ],
)

Time increases along the horizontal axis.
The aircraft that are sequenced are laid out vertically, from the first to land or take off at the top to the last at the bottom.
The black mark the landing or take-off times and the de-icing times (for departures) while the dashed ones mark the base times for each aircraft.
The grey background spans the journey of a departure from the beginning of pushback to its eventual take-off.
Different parts of each row can be hovered over to produce tooltips displaying the pushback duration, taxi duration, runway hold duration, lineup duration, and delay.

Although simple, this output is helpful for obtaining a better view and understanding of the solutions generated by the branch-and-bound program or the mathematical program.
It was also invaluable in finding and eliminating bugs in both implementations during development.

= Results <section:results>

This section presents the results (and their impact) of evaluating both the mathematical program as well as the branch-and-bound algorithm.
As mentioned in @section:introduction, the integrated approach and both decomposed approaches implemented by the latter are compared on the basis of their resulting objective values, runway utilisation, and runway hold times, as well as their runtimes.
The mathematical program and the branch-and-bound program -- both using integrated de-icing -- are also compared using a subset of the problem instances presented in @section:problem-instances.

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

== Problem Instances <section:problem-instances>

The performance of the CPLEX model and the branch-and-bound program (utilising the three different de-icing approaches) is illustrated here using complex real-world problem instances from a single day of departure operations at London Heathrow -- whose characteristics are summarised in @table:heathrow-instances -- as well as benchmark problem instances from Milan Linate.
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

These problem instances reflect realistic use cases and have a variety of different characteristics.
For example, the problem instances from London Heathrow are highly complex, with up to six different SID routes in use at any given time and up to five different weight classes to consider.
This results in a complex separation matrix, in which triangle inequalities are often violated -- i.e. the runway separation for an aircraft is influenced by multiple preceding aircraft rather than just the immediately preceding aircraft @demaere-pruning-rules.
Additionally, a substantial number of aircraft are also subject to CTOTs, which further reduces the number of complete orders that can be inferred.

By contrast, the problem instances from Milan Linate are significantly simpler due to having a relatively high number of separation-identical aircraft and a mix of both arrivals and departures, which allows complete orders to be inferred between a relatively large number of aircraft in each instance.

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

@table:branch-bound-heathrow-results lists the makespans, earliest and latest de-icing times, objective values, and total runway hold times for all Heathrow problem instances solved by the branch-and-bound program utilising the three different de-icing approaches.
The small problem instances were solved without a rolling horizon, while a rolling horizon of 10 was used for the medium and large instances.
Entries for runs that fail to produce feasible solutions are left blank.

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

It can be observed from @table:branch-bound-heathrow-results that the two different decomposed de-icing approaches -- by TOBT and by CTOT -- result in nearly identical makespans, earliest and latest de-icing times, objective values, and runway hold times across all problem instances, with the latter attaining only a #calc.round((heathrow-improvements.tobt-ctot - 1.0) * 100, digits: 2)% improvement in objective values on average compared to the former.

However, integrated de-icing achieves #calc.round((heathrow-improvements.tobt-integrated - 1.0) * 100, digits: 2)% and #calc.round((heathrow-improvements.ctot-integrated - 1.0) * 100, digits: 2)% better objective values on average compared to decomposed de-icing by TOBT and by CTOT respectively.
It also produces shorter makespans in the larger problem instances, indicating better runway utilisation over time compared to its decomposed counterparts -- even when using a rolling horizon.

Additionally, integrated de-icing results in considerably lower runway hold times (and thus higher stand holding times) for every single instance, although it should be noted that neither decomposed approach specifically optimises for stand holding.

@chart:branch-bound-heathrow-avg-runtimes further shows the mean runtime for each individual problem instance solved using each de-icing approach.
It can be seen that decomposed de-icing is faster than integrated de-icing for small- and medium-sized problem instances, but slower for large instances.

#let branch-bound-heathrow-benches = (
    tobt: benches("benches/heathrow/deice-tobt/", range(1, 20 + 1) + range(26, 30 + 1)),
    ctot: benches("benches/heathrow/deice-ctot/", range(1, 20 + 1) + range(26, 30 + 1)),
    integrated: benches("benches/heathrow/deice-integrated/", range(1, 30 + 1)),
)

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
        Mean runtimes for each problem instance from London Heathrow solved by the branch-and-bound program using each de-icing approach
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

// TODO: Recheck the accuracy of these numbers
Moreover, both decomposed approaches failed to produce feasible solutions for instances 21 through 25, whereas the integrated approach was able to do so.
Further testing reveals that a rolling horizon of 20 or higher is required to solve these instances using decomposed de-icing; however, the resulting objective values and mean runtimes are still worse than those achieved by the integrated approach using a lower rolling horizon of 10.
As such, despite having solved less instances, both decomposed de-icing approaches have higher total (and average) runtimes than that of the integrated approach.
A breakdown of the total and average runtimes across each instance size group (small, medium, and large) is shown in @chart:heathrow-total-avg-runtimes.

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

#let branch-bound-furini-results = results-table(
    group-headers: ([Decomposed de-icing], [Integrated de-icing]),
    side-headers: true,
    results.furini.branch-bound.decomposed,
    results.furini.branch-bound.integrated,
)

#figure(
    branch-bound-furini-results,
    caption: [
        Results for the Milan benchmark problem instances introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program using each de-icing approach
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

@table:branch-bound-furini-runtime-stats provides a more detailed overview of the runtimes of each de-icing approach for each problem instance from Milan Linate, including the mean runtime, standard deviation $sigma$, median runtime, and median absolute deviation.

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

== Comparison of Programs <section:compare-programs>

@table:cplex-branch-bound-heathrow-results lists the makespans, earliest and latest de-icing times, and total runway hold times for all small instances from London Heathrow, solved using the mathematical program implemented in CPLEX as well as the branch-and-bound program -- both utilising an integrated de-icing approach.
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

Although not shown in @table:cplex-branch-bound-heathrow-results, the solutions produced by the mathematical program are in some cases different to those produced by the branch-and-bound program, albeit with the same objective values.
This is due to symmetries in the problem, leading to solutions where the order of certain aircraft with the same base times, time windows, and CTOT slots is swapped around with no change to the total delay or CTOT violations.

@chart:cplex-heathrow-avg-runtimes shows the mean runtime for each individual problem instance solved by CPLEX as well as by the branch-and-bound program, both using integrated de-icing.
The runtimes for the latter are already reported in @chart:branch-bound-heathrow-avg-runtimes and @table:branch-bound-furini-runtime-stats, but are shown here again for ease of comparison.

#let cplex-heathrow-benches = {
    let data = csv("benches/cplex.csv").slice(1)
    data
        .enumerate()
        .fold((:), (dict, (idx, (mean, std-dev, median, mad))) => {
            let id = str(idx + 1)
            dict + (
                (id): (
                    stats: (
                        mean: (point-estimate: float(mean) * 1000000000),
                        std-dev: (point-estimate: float(std-dev) * 1000000000),
                        median: (point-estimate: float(median) * 1000000000),
                        median-abs-dev: (point-estimate: float(mad) * 1000000000),
                    ),
                ),
            )
        })
}

#let cplex-heathrow-avg-runtimes = {
    let branch-bound-heathrow-small = branch-bound-heathrow-benches
        .integrated
        .pairs()
        .filter(((key, ..)) => key in cplex-heathrow-benches)
        .fold((:), (dict, ((key, val))) => dict + ((key): val))

    avg-runtime-graph(
        size: (12, 8),
        y-grid: true,
        y-min: 1,
        y-max: 8,
        legend: "legend.inner-north-west",
        ([CPLEX], cplex-heathrow-benches, palette.red(3).fill),
        ([Branch-and-bound], branch-bound-heathrow-small, palette.blue(3).fill),
    )
}

#figure(
    cplex-heathrow-avg-runtimes,
    caption: [
        Mean runtimes for each small problem instance from London Heathrow solved by CPLEX as well as the branch-and-bound program using integrated de-icing
    ],
) <chart:cplex-heathrow-avg-runtimes>

It can be clearly seen that the mathematical program implemented in CPLEX is many orders of magnitude slower than the branch-and-bound program.

// TODO: Compare CPLEX runtimes with branch-and-bound runtimes

@table:cplex-heathrow-runtime-stats provides a more detailed overview of the runtimes of running CPLEX on all small problem instances from London Heathrow, including the mean runtimes, standard deviations $sigma$, median runtimes, and median absolute deviations.

#let cplex-heathrow-runtimes = results-table(
    group-headers: ([*Integrated de-icing*],),
    side-headers: true,
    runtime-stats(cplex-heathrow-benches),
)

#figure(
    cplex-heathrow-runtimes,
    caption: [Mean, standard deviation, median, and median absolute deviation of runtimes for each small problem instance from London Heathrow solved by CPLEX using the integrated de-icing approach],
) <table:cplex-heathrow-runtime-stats>

== Impact <section:impact>

The results discussed in @section:compare-deice clearly show that for the objective values achieved by integrated de-icing are no worse than those achieved by decomposed de-icing for all problem instances considered, and are often significantly better.
@table:cplex-branch-bound-heathrow-results confirms that integrated de-icing indeed achieves optimal values for the objective function considered here, when applied without a rolling horizon.

The results reported in @table:branch-bound-heathrow-results for real-world problem instances from London Heathrow show that significant improvements in total delay and CTOT compliance can be obtained when using integrated de-icing as opposed to decomposed de-icing, while also not compromising on runway utilisation and stand holding times.

By contrast, integrated de-icing sometimes results in slightly worse runway utilisation and higher runway holding times for the problem instances from Milan Linate, as seen in @table:branch-bound-furini-results.
Nevertheless, the improvement in objective values obtained from using integrated de-icing is nearly #calc.round(furini-integrated-improvement / heathrow-improvements.tobt-integrated) times as much as the improvements reported for the Heathrow instances, and outweighs the increase in makespan and runway holding times.

The branch-and-bound implementation using de-icing is slower than decomposed de-icing for most problem instances considered in @section:compare-deice -- namely the Milan problem instances and the small- and medium-sized Heathrow instances.
However, the mean runtime for each de-icing approach -- presented in @table:branch-bound-heathrow-runtime-stats and @table:branch-bound-furini-runtime-stats -- is still under one second for all problem instances.
In some cases, such as for the large Heathrow problem instances, integrated de-icing yields solutions substantially quicker than decomposed de-icing.

It should also be noted that the branch-and-bound implementation outlined in @code:branch-bound does not utilise any parallelisation.
However, it is possible to parallelise this algorithm, which may yield further reductions in runtime for all de-icing approaches.

These computational results thus indicate that using an integrated de-icing approach does not affect the tractability of runway sequencing, and that it is viable for real-time applications with strict limits on computation time.

= Reflections <section:reflections>

This project has been an incredibly enjoyable experience for me as a whole.
It is incredibly satisfying to see what I have accomplished since when I began working on it -- especially considering that I was not at all familiar with operations research and mathematical modelling at the time.
In the process of conducting research and reviewing the literature on runway sequencing and machine scheduling, I have gained a deeper and more thorough understanding of operations research, combinatorial optimisation, mathematical modelling, and artificial intelligence methods like branch-and-bound.
It has also furthered my interest in these fields, and opened new avenues for further exploration.

== Project Management

I approached this project in a fairly iterative and flexible manner, following a general schedule I created in the first few weeks of the first semester.
I had weekly discussions with my supervisor, which helped enforce this iterative approach, and allowed feedback to be obtained regularly and incorporated as soon as possible without adversely impacting the schedule.
These weekly meetings also provided me with the motivation to consistently and regularly work on on the project.
Additionally, I made my schedule as general and broad as possible in order to allow for ample flexibility, which worked very well with the project's iterative nature and proved to be beneficial down the line.

During the first semester, developing an initial working branch-and-bound implementation took me longer than I had anticipated, leading to some delay in my schedule.
I had initially allocated two weeks for this task; however, I required closer to three weeks to complete it, since I first needed to acquire a sound understanding of the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose") as well as approach used by #cite(<psaraftis-dynamic-programming>, form: "prose") to reduce the complexity of runway sequencing.
In hindsight, I also spent more time than necessary on writing the project proposal, which left me with less time for other tasks.

Thankfully, I foresaw such delays and accounted for them with gaps in my schedule to serve as buffer periods.
These were incredibly valuable as they allowed me to extend and adapt my schedule in a flexible manner without delaying the tasks that were remaining to be completed.
Additionally, some of the later tasks -- such as developing the sequence visualiser and incorporating a rolling horizon into the branch-and-bound program -- were completed ahead of schedule, which further offset the delays and enabled me to stay on schedule for the rest of the project.
Towards the end of the first semester, I was able to revise my original schedule to be more realistic based on the progress I had made so far.

A large portion of the second semester was spent on acquiring an understanding of mathematical programming, as this project was my first foray into mathematical programming and operations research.
As part of this, my supervisor initially suggested I enrol into a module focused on discrete optimisation.
However, I wished to explore other modules focused on machine learning and compilers, which did not leave me with enough credits for the discrete optimisation module.
The resources for mathematical modelling and CPLEX shared by my supervisor were extremely helpful in providing me with a foundation in discrete optimisation, and as such, I decided not to enrol in the module.
One of my key takeaways from these resources was the use of sparse data to create lean and efficient mathematical models in CPLEX, which enabled me to improve the tractability of my mathematical program.

I was able to meet all the goals I had initially set out for this project, though I was unable to implement a DP algorithm for integrated runway sequencing and de-icing, which was a stretch goal I had mentioned in both my project proposal and interim report.
Although it may have been possible to experiment in this area during the final few weeks of the second semester, I decided to focus on writing my dissertation and polishing my existing code and implementations instead.

Overall, I was able to create a realistic and flexible project schedule, and best estimate the required time and effort for most tasks, leading to me accomplishing all the objectives I had set out to achieve on time.

== Contributions

The development of new runway sequencing approaches that are able to generate optimal solutions to the problem in more efficient ways than before directly benefits airports and the aviation industry.
The model and algorithms presented here enable further reductions in delay and CTOT compliance and improve runway utilisation, which by extension reduce operating costs.
Additionally, the reduction in delay has an indirect impact on passengers, saving their time as well.
The equitable distribution of delay achieved by the cost function discussed in @section:delay further ensures that aircraft are not disproportionately delayed due to circumstances beyond their control, and avoids unintentionally favouring or being biased towards certain aircraft or airlines.

With regards to the United Nations' Sustainable Development Goals, this project has the potential to indirectly contribute to sustainable development.
As already mentioned in @section:compare-deice, the branch-and-bound algorithm optimises for stand holding when using an integrated de-icing appraoch, minimising runway holding times and consequently fuel consumption from runway holding.
This is a significant improvement over approaches that do not consider runway holding times or allocate large amounts of runway holding times without optimising for stand holding.

Some private data was used for the evaluation of the model and branch-and-bound algorithm presented in this dissertation -- namely the problem instances from London Heathrow.
This data was only provided when necessary, and was stored on an encrypted hard drive accessible only to the author.
Additionally, care was taken to prevent the data from being uploaded to a publicly accessible repository, or from being included in the executable files produced when compiling the project.
This ensured the confidentiality of the data throughout the process.

== Future Developments

This project has achieved all the objectives set out within the given timeframe.
However, there is scope for further research, which can be incorporated into the model and algorithms presented here to make them more realistic or efficient.

An obvious extension would be to consider multiple runways and multiple de-icing stations.
Although both are already supported in limited capacity by pre-allocating the specific runway and de-icing station for each aircraft, factoring in multiple runways and multiple de-icing stations would generalise the model.
It would also allow the model to find optimal runway and de-icing configurations that may not be attainable when pre-allocating the runways and de-icing stations.

However, this would also increase the computational complexity of the problem, possibly beyond what the branch-and-bound algorithm in @section:branch-bound can currently handle.
There would likely be a need to incorporate more of pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose") into the algorithm to improve its tractability.
Alternative approaches such as a metaheuristic search or a DP algorithm could also be explored and compared with the MIP formulation and branch-and-bound algorithm.

Furthermore, it would be interesting to investigate the effects of including runway holding times into the objective function.
Ideally, this would improve the realism of the model by assigning a cost for the fuel consumed during runway holding, and allow greater control and customisation over how runway holding times are allocated.
However, modelling runway holding times as an objective would no longer permit the application of some of the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose") due to the nature of the (new) objective function.
There would thus be a need to re-investigate pruning rules with the inclusion of runway holding times.

// TODO: Write more about future directions if necessary

= Conclusion <section:conclusion>

To summarise, this dissertation introduced a novel mathematical model for the integrated runway sequencing and de-icing problem, and a branch-and-bound algorithm capable of solving the problem to optimality.
A number of techniques from the literature on machine scheduling and runway sequencing were applied to exploit the fundamental characteristics of the problem and improve the efficiency of both the 0-1 MIP formulation and the branch-and-bound program.
A rolling horizon extension was also developed in order to improve the tractability of the latter.

Both the branch-and-bound program and the MIP formulation were then evaluated using real-world problem instances from two major airports in Europe -- namely London Heathrow and Milan Linate -- with vastly different problem characteristics.
This involved comparing the integrated de-icing approach against two different decomposed approaches, which sort aircraft by TOBT and by CTOT respectively and generate a de-icing queue first before solving the runway sequencing problem.

The results show that using an integrated de-icing approach over either of the decomposed approaches leads to significant improvements in total delay and CTOT compliance, while still achieving similar -- and in some cases, better -- runway utilisation and stand holding times.
When combined with a rolling horizon heuristic, the branch-and-bound algorithm with integrated de-icing is able to produce solutions well within the tight time restrictions imposed by real-time applications -- in some cases, it is even faster than the decomposed approaches using the same rolling horizon.
Integrated de-icing is thus a viable and attractive approach for many airports around the world dealing with large and difficult runway sequencing problem instances.

= References

// NOTE: Title disabled since we want to use a custom title and passing in a heading as the title makes
//       it too big and messes up the table of contents
#bibliography("references.yml", title: none, style: "ieee")
