# **CHAPTER 11** **FREEWAY RELIABILITY ANALYSIS** **CONTENTS**

**1. INTRODUCTION**

Overview

Chapter Organization

Related HCM Content


**2. CONCEPTS**

Overview

Freeway Travel Time and Reliability

Scenario Generation

Active Traffic and Demand Management


**3. METHODOLOGY**

Scope of the Methodology

Required Data and Sources

Methodology Overview

Computational Steps


**4. EXTENSIONS TO THE METHODOLOGY**

Active Traffic and Demand Management


**5. APPLICATIONS**


Example Problems

Example Results

Default Values

Planning, Preliminary Engineering, and Design Analysis

Use of Alternative Tools


**6. REFERENCES**

# **LIST OF EXHIBITS**


Exhibit 11-1: Schematic Representation of Freeway Reliability Analysis
Time–Space Domain

Exhibit 11-2: Illustrative Probability Density and Cumulative Distribution
Functions of Travel Time on a Freeway Facility

Exhibit 11-3: Derivation of Time-Based Reliability Performance Measures
from the Travel Time Distribution

Exhibit 11-4: Schematic of the Freeway Scenario Generation Process and
Influential Factors

Exhibit 11-5: Scenario Illustrating Weather and Incident Events

Exhibit 11-6: Process Flow for ATDM Implementation for Freeway
Facilities

Exhibit 11-7: Freeway Reliability Methodology Framework

Exhibit 11-8: Overview of Reliability Parameters

Exhibit 11-9: Recommended Number of Replications for Scenario
Generation

Exhibit 11-10: Required Input Data, Potential Data Sources, and Default
Values for Freeway Reliability Analysis

Exhibit 11-11: Freeway Reliability Methodology Framework

Exhibit 11-12: Freeway ATDM Strategy Evaluation Framework


Exhibit 11-13: List of Example Problems

Exhibit 11-14: Illustrative Effects of Different Nonrecurring Sources of
Congestion on the TTI Distribution

Exhibit 11-15: Illustrative Effects of Inclement Weather Events on the TTI
Distribution

Exhibit 11-16: Illustrative Effects of Incident Rates on the TTI Distribution

Exhibit 11-17: Effect of Activating Hard Shoulder Running ATDM Strategy

Exhibit 11-18: Default Urban Freeway Demand Ratios (ADT/Mondays in
January)

Exhibit 11-19: Default Rural Freeway Demand Ratios (ADT/Mondays in
January)

Exhibit 11-20: Default CAFs by Weather Condition

Exhibit 11-21: Default SAFs by Weather Condition

Exhibit 11-22: Default Freeway Incident Severity Distribution and Duration
Parameters (min)

Exhibit 11-23: CAFs by Incident Type and Number of Directional Lanes on
the Facility

Exhibit 11-24: Input Data Needs for HCM Planning Reliability Analysis of
Freeways


# **1. INTRODUCTION**

**OVERVIEW**

This chapter provides a methodology for evaluating a freeway’s travel
time reliability over a multiday or multimonth _reliability reporting period_
(RRP). The methodology estimates the impacts of recurring and nonrecurring
congestion (i.e., demand variations and fluctuations, incidents, weather, work
zones, and special events) on the travel time distribution over the course of
the RRP. The methodology can be extended to estimate the impacts of active
traffic and demand management (ATDM) strategies on the travel time
distribution.

The methodology relies on the freeway facilities core methodology
presented in Chapter 10, which in turn applies the freeway segment
methodologies in Chapters 12, 13, and 14. The freeway facilities core
methodology focuses the analysis on a single day or less, while the segment
methodologies are limited to the analysis of one 15-min period. In contrast,
this chapter’s methodology is capable of applying the core method repeatedly
across multiple days, weeks, and months, up to a 1-year RRP. A 1-year RRP
is the most common application, although shorter periods are possible for
specific applications (e.g., reliability of summer tourist traffic, a focus on the
construction season). RRPs longer than 1 year are uncommon, since most
typical variations in travel time (day of week, month of year, weather, and
incidents) are encapsulated in a single year.

The methodology is integrated with the FREEVAL-2015E computational
engine, which implements the complex computations involved. This engine
was developed to test the methodology; other software implementations are
available. This chapter discusses the basic principles of the methodology and
its application. Chapter 25, Freeway Facilities: Supplemental, provides a
detailed description of all the algorithms that define the methodology.


**CHAPTER ORGANIZATION**


Section 2 of this chapter presents the basic concepts of freeway
reliability analysis, including performance measures derived from the travel
time distribution. The section also provides an introduction to scenario
generation concepts and evaluation of ATDM strategies in the context of this
chapter.

Section 3 presents the base methodology for evaluating freeway
reliability. The method generates a series of performance measures that can
be derived from the travel time distribution, including various percentile
travel time indices and on-time performance ratings.

Section 4 extends the core method presented in Section 3 to the
evaluation of ATDM strategies in a travel time reliability context.

Section 5 presents guidance on using the results of a freeway facility
analysis, provides example results from the methods, discusses planninglevel reliability analysis, and provides guidance on the use of alternative
tools.


**RELATED HCM CONTENT**

Other _Highway Capacity Manual_ (HCM) content related to this chapter
includes the following:

   - Chapter 3, Modal Characteristics, where the motorized vehicle
methodology’s Variations in Demand subsection describes typical
travel demand patterns for freeway and multilane highway segments;

   - Chapter 4, Traffic Operations and Capacity Concepts, which
provides background for the refinements specific to freeway and
multilane highway segments that are presented in this chapter’s
Section 2;

   - Chapter 10, Freeway Facilities Core Methodology, which forms the
basis for this chapter’s computations in a single-day application;

   - Chapters 12, 13, and 14, which present the segment methodologies
for basic freeway segments, freeway weaving segments, and freeway
merge and diverge segments, respectively;

   - Chapter 25, Freeway Facilities: Supplemental, which provides the
computational details of this chapter’s methodology, including a


detailed description of the scenario generation procedure;

- Chapter 37, ATDM: Supplemental, which provides additional details
and concepts related to ATDM strategy types and their expected
operational impacts; and

- Section H, Freeway Analyses, in the _Planning and Preliminary_
_Engineering Applications Guide to the HCM,_ found in Volume 4,
which describes how to incorporate this chapter’s methods and
performance measures into a planning effort.


# **2. CONCEPTS**

**OVERVIEW**

Freeway travel time reliability reflects the distribution of the travel times
for trips traversing an entire freeway facility over an extended period of
time, typically 1 year, during any portion of the day. A 1-year RRP is typical,
since it covers most variation in travel times arising from the factors below.
Shorter RRPs are possible for special circumstances, such as a focus on
summer tourist travel or the work zone construction season.

The travel time distribution is created by the interaction of several
factors that influence facility travel times:

   - _Recurring variations in demand_ by hour of day, day of week, and
month of year; within certain limits, these variations are more or less
predictable;

   - _Severe weather_ (e.g., heavy rain, snow, poor visibility) that reduces
speeds and capacity and may influence demand; this is a nonrecurring
event;

   - _Incidents_ (e.g., crashes, disabled vehicles, debris) that reduce
capacity; these are nonrecurring events;

   - _Work zones_ that reduce capacity and—for longer-duration work
activities—may influence demand; these are nonrecurring events; and

   - _Special events_ that produce temporary intense traffic demands, which
may be managed in part by changes in the facility’s geometry or
traffic control; special events can be scheduled or recurring (e.g., a
state fair) or nonrecurring (e.g., concerts).

As explained in the Travel Time Reliability section of Chapter 4, Traffic
Operations and Capacity Concepts, the underlying distribution of travel times
expresses the _variability_ in travel times that occur on a facility or a trip over
the course of time, as expressed by 50th, 80th, and 95th percentile travel


times and other distribution metrics. The travel time observations in the
distribution are the _average_ facilitywide travel times over a 15-min period,
not individual vehicle travel times.

ATDM for freeways consists of the dynamic and continuous monitoring
and control of traffic operations to improve facility performance. Examples
of freeway ATDM measures are managed lanes, dynamic ramp metering,
incident management, changeable message signs, hard shoulder running, and
speed harmonization (variable speed limits). ATDM strategies are discussed
in detail in Chapter 37, ATDM: Supplemental.

ATDM measures can influence both the nature of demand on the freeway
facility and the ability of the facility to deliver the capacity and quality of
service tailored to serve the demand. Combining reliability and ATDM in
this chapter is natural, since the ATDM toolbox serves to mitigate
nonrecurring congestion in a near-real-time, dynamic response mode.

In a highway capacity analysis context, the effects of both ( _a_ ) factors
affecting travel time reliability (e.g., weather, incidents) and ( _b_ ) ATDM
strategies are modeled as variations in (or adjustments to) one or more
parameters used in a freeway facilities analysis. These parameters are
adjusted during specific analysis periods on specific affected freeway
segments. These parameters include

   - Number of mainline lanes open to traffic;

   - Available capacity per freeway lane that is open to traffic;

   - Facility free-flow speed;

   - On-ramp capacity or throughput;

   - Demand flow rates at origin points, destination points, or both;

   - Incident frequencies; and

   - Incident clearance times.


**FREEWAY TRAVEL TIME AND RELIABILITY**


**Objectives for Reliability Analysis**


An important step in any analysis is defining _why_ the analysis is being
performed. Key questions or issues should be defined, performance measures
that help answer those questions identified, and a basis of comparison for
interpreting the analysis results established. Reliability analysis is no
different. The following are examples of potential objectives of a reliability
analysis:

   - _Tracking_ the reliability of a set of freeway facilities in a jurisdiction
or region over an extended period to prioritize operational or
physical strategies intended to improve reliability;

   - _Diagnosing_ the primary causes of the reliability problems on a given
facility so that an improvement program can be developed and
specific strategies applied to enhance reliability; and

   - _Predicting_ the effects of a particular treatment or improvement
strategy on a facility, including testing the effectiveness and benefit–
cost of ATDM strategies.

More broadly, travel time reliability analysis can be used to improve the
operation, planning, prioritization, and programming of transportation
improvement projects in the following applications: long-range
transportation plans, transportation improvement programs, corridor plans,
major investment studies, congestion management, operations planning, and
demand forecasting.

Reliability analyses can often also be performed by using field data
gathered through the use of sensors and stored in long-term speed and travel
time archives increasingly available to many transportation agencies. The
HCM reliability method can be used to supplement these field sources and is
particularly valuable in evaluating and testing strategies intended to improve
reliability, as discussed in the bullets above. Field data can also be used to
validate the HCM method, but the method described in this chapter is
uniquely suited for evaluating trade-offs and the benefit–cost relationship of
different strategies intended to make a facility more reliable.


**Reliability Methodology Definitions**

Conceptually, travel time reliability can be viewed as an extension of the
freeway facilities core methodology presented in Chapter 10. The extension


occurs in the time dimension, by transitioning from a “typical day” or “single
study period” analysis to a reliability dimension, which is an extendedperiod analysis covering several days, weeks, months, or a full year. This
new dimension gives rise to the following set of definitions, many of which
are illustrated in Exhibit 11-1:

   - _Travel time._ The time required for a vehicle to travel the full length
of the freeway facility from mainline entry point to mainline exit point
without leaving the facility or stopping for reasons unrelated to traffic
conditions.

   - _Free-flow travel time._ The facility’s length divided by its free-flow
speed.

   - _Travel time index (TTI)._ The ratio of the actual travel time to the
free-flow travel time. By definition, TTI is always greater than or
equal to 1.0. The TTI’s distribution is identical to that of travel time,
except that its values are indexed to the free-flow travel time.

   - _Percentile travel time index (TTIpp)._ Represents the _pp_ percentile
TTI in the travel time distribution. For example, _TTI_ 85 means that this
observation is exceeded only 15% of the time in the travel time
distribution. Common TTI percentiles are _TTI_ 50 (the median TTI) and
_TTI_ 95 (the 95th percentile TTI). When _pp_ is omitted, the value often
represents the _mean_ TTI for the distribution, which in this chapter is
referred to as _TTI_ mean.

   - _Analysis segment._ An HCM freeway segment (e.g., basic, merge,
diverge, weaving) as described in Chapters 12 through 14. Each
column in Exhibit 11-1 represents an analysis segment.


**Exhibit 11-1: Schematic Representation of Freeway Reliability Analysis Time–Space**
**Domain**


Source: Zegeer et al. ( _1_ ).


   - _Analysis period._ The time interval evaluated by a single application
of an HCM methodology (15 min for the freeway facilities core
methodology). In Exhibit 11-1, there are 12 such analysis periods for
the facility, represented by the rows in the rectangles. Each cell in a
rectangle represents a single analysis period for a single analysis
segment.

   - _Study period._ The time interval within a day for which facility
performance is evaluated. It consists of one or more consecutive
analysis periods, represented by the rows in the rectangles in Exhibit
11-1. In this example, the study period is 3 h long, from 4 to 7 p.m.
(i.e., 16:00 to 19:00 hours).

   - _Scenario._ A single instance of a study period for the facility, with a
unique combination of traffic demands, capacities, geometries, and
free-flow speeds represented in its analysis periods. Each rectangle


in Exhibit 11-1 represents a unique scenario, or in other words 1 day
of the year.

- _Base scenario (seed file)._ A set of parameters representing the
facility’s calibrated operating conditions during one study period. All
other scenarios are developed by adjusting the base scenario’s inputs
to reflect the effects of varying demand, weather, incidents, work
zones, or a combination occurring in other study periods. When the
methodology is executed by using a computational engine, the base
scenario’s parameters become inputs to the seed file used by the
engine.

- _Reliability reporting period._ The specific set of days over which
travel time reliability is computed; for example, all nonholiday
weekdays in a year. The RRP represents the third dimension that
extends the freeway facilities core methodology and is illustrated in
Exhibit 11-1 by the series of rectangles (scenarios).

- _Travel time distribution._ The distribution of average facility travel
times by analysis period across the RRP. Each 15-min analysis
period within each scenario contributes one data point to the travel
time distribution. It is _not_ the distribution of individual vehicle travel
times (or TTIs).

- _Probability density function (PDF) and cumulative distribution_
_function (CDF)._ The PDF gives the number or percent of all
observations within a specified travel time (or TTI) bin. The CDF
gives the number or percent of all observations _at or below_ a
specified travel time bin. Exhibit 11-2 illustrates the two types of
distributions, with the PDF shown by the solid line and the CDF by
the dashed line. The facility travel times shown on the _x_ -axis are the
midpoints of the various travel bins.


**Exhibit 11-2: Illustrative Probability Density and Cumulative Distribution Functions of**
**Travel Time on a Freeway Facility**


**Travel Time Distribution and Reliability Performance Measures**

Exhibit 4-5 in Chapter 4, Traffic Operations and Capacity Concepts,
illustrates how various reliability performance measures can be derived
from the CDF of a travel time distribution. When travel times are measured
or predicted over a long period (e.g., a year), a distribution of travel time
emerges. The following are useful measures for describing ( _a_ ) travel time or
TTI variability or ( _b_ ) the success or failure of individual trips in meeting a
target travel time or speed:

   - _TTI_ 95 (unitless). The 95th percentile TTI is also referred to as the
planning time index (PTI) and is a useful measure for estimating the
added time travelers must budget to ensure an on-time arrival with
“failure” limited to one trip per month. In Exhibit 11-2, the 95th
percentile travel time is 45 min, compared with a free-flow travel


time of 15 min; thus, _TTI_ 95 = 3.0. The planning time is the difference
between the 95th percentile and free-flow travel times, or 30 min.

   - _TTI_ 80 (unitless). Research indicates that this measure is more
sensitive to operational changes than the _TTI_ 95 ( _2_ ), which makes it
useful for strategy comparison and prioritization purposes. In Exhibit
11-2, the 80th percentile travel time is approximately 36 min; thus the
80th percentile TTI is 36/15 = 2.4.

   - _TTI_ 50 and _TTI_ mean (unitless). These measures describe the median
and mean of the TTI distribution, respectively. Both can be useful
measures, with the median being less influenced by outliers than the
mean.

   - _Failure_ or _on-time measures_ (percentage). The percentage of
analysis periods with space mean speeds above (on time) or below
(failure) one or more target values (e.g., 35, 45, and 50 mi/h). These
measures address how often trips succeed or fail in achieving a
desired travel time or speed.

   - _Reliability rating_ (percentage). The percentage of vehicle miles
traveled (VMT) on the freeway facility that experiences a TTI less
than 1.33. This threshold approximates the points beyond which
travel times become much more variable (unreliable).

   - _Semi–standard deviation_ (unitless) _._ A one-sided standard deviation,
with the reference point being free-flow travel time (or TTI = 1)
instead of the mean. It reflects the mean variability from free-flow
conditions.

   - _Standard deviation_ (unitless). The standard statistical measure.

   - _Misery index_ (unitless). A measure comparing the average of the
worst 5% of travel times with the free-flow travel time.

The travel time distribution and some of its key performance measures
are illustrated in Exhibit 11-3.


**Exhibit 11-3: Derivation of Time-Based Reliability Performance Measures from the**
**Travel Time Distribution**


Source: Zegeer et al. ( _1_ ).


**SCENARIO GENERATION**

As the freeway facilities core methodology is expanded from a single
study period (or representative condition) to capture variations in
performance across the RRP, generation of scenarios describing how
operations are affected by combinations of changes in demand, weather,
incidents, and scheduled work zones becomes necessary. These factors are
facility-specific. Weather depends on geographic location, incidents on
congestion and incident management levels, work zone on infrastructure
quality, facility demand on characteristics of the facility’s travel patterns, and
so on.

The process of enumerating the various combinations of these factors and
calculating their probability of occurrence is termed _freeway scenario_
_generation._ The scenario generation process is described conceptually in
this section, and the step-by-step procedures for implementing freeway
scenario generation are described in Chapter 25, Freeway Facilities:
Supplemental.


The calendar creates an important connection between all the factors
influencing travel time reliability. Weather is intuitively calendar-based (e.g.,
more snow falls in the winter than in the summer), as are traffic demand
patterns to a great extent. Work zones, at least in areas with inclement winter
weather, are typically scheduled to avoid extreme weather events.
Furthermore, the number of incidents is likely to be directly correlated with
traffic demand and thus indirectly tied to the calendar.

The mechanism of implementing freeway scenario generation is actually
simple. On the basis of the analyst’s input of influential factors (e.g., how
facility demand varies over time, how weather events vary on a monthly
basis), the scenario generation process takes the input events and generates a
combination of scenarios matching those inputs. All scenarios originate with
the base scenario (seed file), whose inputs are manipulated via changes in
free-flow speed, individual segment capacity, lane losses, and (possibly)
demand changes to create a new unique combination of events, or scenario.
A high-level schematic of the freeway scenario generation process is
depicted in Exhibit 11-4.


**Exhibit 11-4: Schematic of the Freeway Scenario Generation Process and Influential**
**Factors**


The _default_ number of scenarios generated in this procedure, without
considering weekends, is 240 (the parameter _N_ in Exhibit 11-4). This value
was obtained by creating four replications of each weekday–month demand
combination (5 weekdays × 12 months × 4 replications). Having multiple
replications of the weekday and month combination ensures inclusion of a
sufficiently large sample of weather and incident events in the reliability
analysis. Any stochastic scenario effects (e.g., weather or incidents) will
vary across these four replications. Specific guidance for the number of
replications as a function of the length of the RRP is given in Section 3, but
the choice of four replications roughly corresponds to having each day of the
week appear four times per month (e.g., approximately four Mondays in
January). This process allows the procedure to produce a number of
representative scenarios sufficient to accommodate the variability in all four
factors affecting reliability. The analyst may increase the number of
replications, since that parameter value in the procedure can be controlled by


the user. This approach would be advisable if the RRP is short (e.g., a few
weeks).


**Scenario Generation Approach**

The scenario generation procedure presented in this chapter is a _hybrid_
approach, with some inputs being treated in a deterministic fashion and
others being stochastic in nature. Traffic demand and scheduled work zones
are treated in a deterministic manner. Direct calendar data are used to
characterize demand variability (i.e., day of week, month of year), and userdefined work zone schedules are applied. On the other hand, weather and
incidents are modeled in a stochastic fashion and are assigned randomly to
scenarios. The assignment is based on predefined distributions of weather
and incidents that the analyst specifies to describe the facility. When such
data are not available or are incomplete, the method provides national
default distributions to assist with the scenario generation process.

The objective of the scenario generation process is to maximize the
match (or minimize the difference) between the generated scenarios and the
input distributions of the factors affecting reliability, as entered by the user.
This is accomplished by assigning the correct traffic demand levels, weather
events, and incidents within the different scenarios. Eight distributions are
entered into the freeway scenario generation procedure ( _1_ ):

1. Temporal distribution of traffic demand levels,

2. Temporal distribution of weather event frequency,

3. Distribution of average weather event duration by weather event type,

4. Temporal distribution of incident event frequency,

5. Distribution of incident severity (i.e., shoulder, single, or multilane
closures),

6. Distribution of incident duration by incident severity,

7. Distribution of incident event start time in a scenario, and

8. Spatial distribution of incident events across segments of the facility.

Only the first six distributions represent manual inputs by the user, and all
have default values available. Items 7 and 8 in the list are estimated by the


computational engine and do not require user input. Details on all
distributions are provided in Chapter 25.

Thus, the hybrid approach generates scenarios such that all eight
specified distributions match actual conditions, with consideration for the
need to round the number of events (incidents, weather, etc.) to integer values
and to round their durations to the nearest 15-min analysis period.


**Treatment of Factors Affecting Travel Time Reliability**

This section provides a high-level description of how each of the four
factors involved in the reliability analysis—demand, weather, incidents, and
work zones—is treated in the scenario generation process.


_Traffic Demand_

The methodology accounts for demand variability by adjusting the traffic
demands for the analysis periods included in the various scenarios. This is
done through the use of a _demand multiplier,_ which is the ratio of the daily
(weekday–month combination) facility demand to the average daily traffic
(or to any combination of day of week and month of year). A second
adjustment is needed to factor the demand measured on the specific day–
month combination _in the base scenario_ to any other day–month combination
in the year. Traffic demand variation for different hours of the day is already
accounted for in the base scenario obtained from the Chapter 10 core facility
analysis.

For example, if the _base scenario_ demand data were gathered on a
Monday in January that has a demand multiplier of 0.85 and a demand
scenario is being tested on a Friday in June that has a demand multiplier of
1.10, the base scenario demands should be factored by a ratio of 1.10/0.85 =
1.29 to create the demand profile for that Friday-in-June scenario. If all days
of the week are considered, there could be up to 84 demand combinations;
for weekday-only analyses, there could be up to 60 demand combinations.


_Weather Events_

Weather events are generated on the basis of their probability of
occurrence during a given month. The scenario generation process accounts
for 10 categories of severe weather events that have been shown to reduce


capacity by at least 4%, along with a _non–severe weather_ category that
encompasses all other weather conditions and that generates no capacity,
demand, or speed adjustments. Default capacity and speed adjustment factors
for weather events are provided in Section 5 of this chapter.

To capture the actual occurrences of various weather events, the analyst
may use default weather data from any of 101 U.S. metropolitan areas, based
on 2001–2010 weather records. These values are documented in the Volume
4 Technical Reference Library.

Alternatively, the analyst may supply a 12-month by 11-weather-event
matrix (132 total values) of local probabilities of each weather event, along
with the average duration for each event (10 values). As mentioned earlier,
different weather events are assigned stochastically to the various scenarios
in a manner that will match their monthly occurrence based on the site’s
meteorological data.


_Traffic Incidents_

Incidents are generated on the basis of their expected frequency of
occurrence per study period (analysis hours in a day) in a given month on the
facility. The analyst may opt to use default expected incident frequencies,
may supply a facility-specific incident or crash rate, or may supply a 12month table of facility-specific expected frequencies of any incident type.
The incident frequency represents the average number of all incidents
experienced on the facility during the study period and is allowed to vary in
each month.

The method makes the following assumptions about a given incident:

   - The incident start time is assigned stochastically to any analysis
period, which is done automatically by the computational engine;

   - The incident duration is assigned stochastically on the basis of the
severity-defined incident duration distribution;

   - The incident location is assigned stochastically, weighted by the
individual segment VMT; and

   - The incident severity is assigned stochastically on the basis of the
distribution of incident severity.


Default adjustment factors for incidents are provided in Section 5.


_Work Zones_

This portion of the analysis pertains exclusively to scheduled, significant
work zone events. Minor patching and repair activities are not treated as
work zones, but these important activities can be treated as incident events in
the procedure and may be added to the incident tally. Thus, a work zone
constitutes any activity that results in scheduled closures of the shoulder or
one or more travel lanes. Typically, a work zone lasts multiple days or
weeks. In some cases, it involves multiple stages, each with different
shoulder- and lane-closure parameters.

The details of scheduled work zone activities must be entered by the
analyst and cannot be defaulted. A work zone log should be entered in which
the following information is input for each work zone activity that is planned
during the RRP:

   - Calendar days of the start and end dates of the work zone activity,

   - Facility segment(s) and analysis periods affected by the work zone
activity,

   - Portions of the facility cross section affected by closures (i.e.,
shoulder, one-lane, or multiple-lane closures),

   - Type of barrier used to separate traffic from the work activity (i.e.,
concrete or other hard barrier; cones, drums, or other soft barrier),

   - Regulatory speed limit in effect during the work activity, and

   - Lateral separation between traffic and the work zone.

The methodology can accommodate multiple work zone activities, each
with its own sets of inputs. Capacity adjustment factors (CAFs) and speed
adjustment factors (SAFs) for work zones have been developed by national
research ( _3, 4_ ) and are described in Section 4 of Chapter 10, Freeway
Facilities Core Methodology.

A schematic illustration of the time–space domain for a scenario
containing weather and incident events is shown in Exhibit 11-5. The
freeway facility in question consists of 10 analysis segments and is analyzed
over a 3-h study period (12 analysis periods). This scenario contains a rain


event (R) that starts 45 min into the study period and lasts for 45 min.
Weather is assumed to affect the entire facility equally.


**Exhibit 11-5: Scenario Illustrating Weather and Incident Events**


Exhibit 11-5 also shows an incident blocking two lanes of Segment 8 (I2) starting 75 min into the study period. This incident is concurrent with the
rain event in Analysis Period 5, and the incident duration is 1 h. Another
minor incident (I-S) closes the shoulder of Segment 3 in Analysis Period 11.
All shaded cells in Exhibit 11-5 (i.e., combinations of analysis segment and
analysis period) will experience some reduction in capacity and possible
changes in free-flow speed and traffic demand.

When two independent events affect capacity at the same time, their
combined effect is the multiplication of the two CAFs. This would be the
case for Segment 8 in Analysis Period 5, where the product of the rain event
CAF and the incident event CAF would be applied. This is also true if the
example had included a work zone event, which would have likely affected
the CAFs and SAFs for all analysis periods on any segments having work
activity.


**ACTIVE TRAFFIC AND DEMAND MANAGEMENT**

ATDM concepts for freeway facilities are presented in Chapter 37,
ATDM: Supplemental. The concepts presented below pertain to how ATDM
is integrated into the freeway facilities core and reliability methodologies.
The ATDM methodology was initially developed by Federal Highway


Administration (FHWA) research ( _5_ ) and has been adapted to fit within the
HCM’s scenario generation and reliability evaluation methodology.

The ATDM methodology requires the analyst to carry out the freeway
facilities core and reliability analyses before testing any ATDM strategies, as
illustrated in Exhibit 11-6. This sequence is required because many ATDM
strategies are targeted to mitigate the impacts of specific types of recurring or
nonrecurring events. For example, if incident-induced delays are significant,
a strategy could be to deploy or increase the frequency of freeway service
patrols to reduce the capacity impacts of those incidents. Obviously, this
strategy will apply only to scenarios where incidents occur. On the other
hand, a recurring bottleneck at a freeway on-ramp could be mitigated by
implementing a ramp-metering strategy across the whole year. In summary _,_
_any subset_ of the reliability scenarios can be viewed as the “before” case for
ATDM analysis, while the scenarios selected for mitigation via ATDM can
be viewed as the “after” case.


**Exhibit 11-6: Process Flow for ATDM Implementation for Freeway Facilities**


Three types of comparisons are provided in the procedure to quantify the
effects of ATDM strategies on freeway facility operations.

   - The first comparison is done at the _individual scenario level_ . It
allows the effects of specific events and strategies to be evaluated
and can be used as the basis for large-scale ATDM analyses later.
This type of comparison can be used to judge the relative effects of
different ATDM strategies _on a common scenario_ to aid the
decision-making process.

   - The second comparison makes use of _all scenarios selected by the_
_analyst_ in the “after” ATDM subset and evaluates performance
changes between the collection of multiple “before” and “after” sets.
This comparison considers only the scenarios that are included in the
ATDM “after” set and does not consider any other scenarios. For
example, if an “after” ATDM set is applied to 25 scenarios, the


second-level comparison will consider the “before” and “after”
ATDM outputs only for those 25 scenarios. This comparison _does not_
_provide any insights into ATDM impacts on reliability_ .

- The final comparison extrapolates the effects of the ATDM analysis
to the _entire set_ of all reliability scenarios and seeks to answer the
following question: How do ATDM strategies applied to a _selected_
number of scenarios affect reliability performance measures over the
full RRP? Here, the distribution of performance measures for the
entire reliability analysis can be compared with that of the ATDM
“after” set, which effectively treats the reliability scenarios as the
“before” case. For the “after” case, which contains some scenarios
that include ATDM strategies, adjustments to the scenario
probabilities are made to match the original TTI distribution of the
set of reliability scenarios, a process described in detail in Chapter
25. Once this adjustment has been completed, the distributions of
performance parameters and other outputs can be compared and
conclusions formed about the effectiveness of the ATDM strategies.


# **3. METHODOLOGY**

This section describes the methodology for evaluating the travel time
reliability of a freeway facility. It also describes extensions to the freeway
facilities core methodology (Chapter 10) that are required for computing
reliability performance measures.

The freeway reliability methodology is computationally intense and
requires software to implement. The intensity stems from the need to create
and process the input and output data associated with the hundreds of
scenarios considered for a typical RRP. The objective of this section is to
introduce the analyst to the calculation process and to discuss the key
analytic procedures. Important equations, concepts, and interpretations are
highlighted. The computational details of the methodology are provided in
Chapter 25, Freeway Facilities: Supplemental.


**SCOPE OF THE METHODOLOGY**


**Framework**

The freeway reliability methodology includes a base dataset, the scenario
generator, and the core computational procedure from Chapter 10. The
computational procedure predicts travel times for each analysis period in
each scenario. They are subsequently assembled into a travel time
distribution that is used to determine performance measures of interest. These
components are illustrated in Exhibit 11-7.


**Exhibit 11-7: Freeway Reliability Methodology Framework**


Exhibit 11-8 provides an overview of the reliability parameters for
geometry, demand, weather, work zones, and incident events. It describes
how these parameters are treated in the three parts of the scenario generation
process: ( _a_ ) treated deterministically in the base scenario (Chapter 10), ( _b_ )
treated deterministically in scenario generation (Chapter 11), or ( _c_ ) treated
stochastically in scenario generation.


**Exhibit 11-8: Overview of Reliability Parameters**



**Treated**
**Stochastically in**
**Scenario**
**Generation**



**Treated**
**Deterministically in**
**Scenario**
**Generation**



**Reliability**
**Parameter**



**Treated**
**Deterministically in**
**Seed File**



Segmentation, number
Facility geometry of lanes, free-flow NA NA
speed, etc.


Traffic demand level 15-min flow rates
represent 1 day in base
scenario



Variable based on
day of week and
month of year



NA



Weather
events


Work
zones


Incident
events



User input or default
Duration NA NA
values

Stochastically
Start time NA NA assigned to analysis
periods



Stochastically
Start time NA NA assigned to analysis
periods

Stochastically
Location NA NA assigned to
segments



User input or default
Frequency NA
values



Stochastically
assigned to
scenarios



Long term
(entire
RRP)



Work zone duration,
segments, schedule in NA NA
base scenario



Short term
User input in specific
(less than NA NA
scenarios
RRP)



User input or default
Duration NA
values



Stochastically
determined on the
basis of user inputs



User input or default
Frequency NA
values


User input or default
Severity NA
values


Note: NA = not applicable.


_Base Dataset_



Stochastically
assigned to
scenarios

Stochastically
assigned to
scenarios



The base dataset provides all the required input data for the freeway
facilities core methodology described in Chapter 10. Some data are specific
to the freeway facility being studied. These items include, at a minimum, all
segment geometries (both general purpose and managed lanes, if applicable),
free-flow speeds, lane patterns, and segment types, along with base demands
that are typically, but not necessarily, reflective of average [annual average
daily traffic (AADT)] conditions. In addition, the base dataset contains the


input data required for executing this chapter’s reliability methodology.
These data include a demand multiplier matrix, weather, work zones, and
incident events as described later in this section. Most of the reliabilityspecific input data can be defaulted when they are not available locally, but
the analyst is encouraged to supply facility-specific data whenever feasible.


_Scenario Generation_

The scenario generator develops a set of scenarios reflecting conditions
that the freeway facility may experience during the RRP. Each scenario
represents a single study period (typically several hours long) that is fully
characterized in terms of demand and capacity variations in time and space.
The data supplied to the scenario generator are expressed as multiplicative
factors [CAFs, SAFs, and demand adjustment factors (DAFs)] or additive
factors (number of lanes) that are applied to the base free-flow speed,
demand, capacity, and number of lanes.

The scenario generation process includes the following steps:

   - Adjusting the base demand to reflect day-of-week and month-of-year
variations associated with a given scenario;

   - Generating inclement weather events on the basis of their local
probability of occurrence in a given time of year and adjusting
capacities and free-flow speeds to reflect the effects of the weather
events;

   - Generating various types of incidents on the basis of their probability
of occurrence and adjusting capacities to reflect their effects; and

   - Incorporating analyst-supplied information about when and where
work zones and special events occur, along with any corresponding
changes in the base demand or geometry.

The results from these steps are used to develop one scenario for each
study period in the RRP.


_Facility Evaluation_

In the facility evaluation step, each scenario is analyzed with the freeway
facilities core methodology. The performance measures of interest to the
evaluation—in particular, facility travel time—are calculated for each


analysis period in each scenario and stored. At the end of this process, a
travel time distribution is formed from the travel time results stored for each
scenario.


_Performance Summary_

In the final step, travel time reliability is described for the entire RRP.
The travel time distribution is used to quantify a range of variability and
reliability metrics.


**Spatial and Temporal Limits**

The reliability methodology is subject to the same spatial and temporal
limits as the freeway facilities core methodology. The RRP can be as long as
1 calendar year, although shorter periods are possible. A 1-year RRP is most
typical, since it encompasses all day-to-day and month-to-month variability
in demand, as well as all weather and incident effects. However, shorter
RRPs can be used to focus on reliability during specific time periods. The
minimum recommended RRP is 1 month to capture sufficient variability in
demand and other factors.

For a 1-year RRP, the methodology is typically applied with four
replications for each of 5 weekdays (Monday through Friday) and 12 months
in the year, for a total of 240 scenarios. This approach roughly corresponds
to 250 work days in a typical calendar year. A reliability analysis that
includes weekend effects would result in an increased number of scenarios.

For RRPs that are significantly shorter than 1 year, the analyst should
increase the number of replications to ensure a sufficient sample size for
scenario generation. Exhibit 11-9 provides guidance on the recommended
number of replications in such cases.


**Exhibit 11-9: Recommended Number of Replications for Scenario Generation**


**RRP Duration** **Number of Days** **Recommended Number** **Resulting Number of**
**(months)** **Considered** **of Replications** **Scenarios**

1 5 (all weekdays) 48 240
2 5 24 240
4 5 12 240
6 5 8 240
9 5 6 270


12 _[a]_ 5 4 _[a]_ 240 _[a]_

12 2 (weekend only) 10 240

12 7 (all days) _[b]_ 3 252


Notes: RRP = reliability reporting period.

_a_ Default value.
_b_ Not desirable; separation of weekday and weekend reliability analysis is preferred.


For the base scenario provided as part of the base dataset, there is no
limit to the number of analysis periods that can be analyzed. The
computational engine supports an evaluation of a 24-h period. The duration
of the study period should be sufficiently long to contain the formation and
dissipation of all queues. The facility length evaluated should be less than the
distance a vehicle traveling at the average speed can travel in 15 min. This
specification generally results in a maximum facility length between 9 and 12
mi. Longer facilities may be evaluated, but results need to be interpreted
carefully, since the onset of congestion in later analysis periods may be
estimated to occur earlier than field observations would indicate. More
discussion on facility length is provided in Chapter 10.


**Performance Measures**

There are many possible performance measures for quantifying aspects of
the travel time reliability distribution. The following measures, defined in
Section 2 of Chapter 4, Traffic Operations and Capacity Concepts, are among
the more useful for quantifying differences in reliability between facilities
and for evaluating alternatives to improve reliability. All of these measures
are produced by the freeway travel time reliability methodology:

   - _TTI_ 95 (i.e., PTI) (unitless),

   - _TTI_ 80 (unitless),

   - _TTI_ 50 (i.e., median TTI) (unitless),

   - _TTI_ mean,

   - Failure and on-time measures (percentage),

   - Reliability rating (percentage),

   - Semi–standard deviation (unitless),


   - Standard deviation (unitless), and

   - Misery index (unitless).

In addition, all of the performance measures generated by the freeway
facilities core methodology (Chapter 10) are computed for each general
purpose and managed lane segment for each analysis period being evaluated.


**Strengths of the Methodology**

The methodology is capable of estimating the impacts of nonrecurring
congestion (due to demand variability, weather, incidents, work zones, and
special events) on the operational performance of a freeway facility over an
extended RRP—up to 1 year. Because of the computational efficiency of the
HCM freeway facilities core methodology compared with, for example, a
simulation analysis of a freeway facility, a whole-year analysis can be
performed relatively quickly. The following are specific strengths of the
methodology:

   - It is an efficient method for estimating travel time reliability. It can be
applied quickly several hundred times to derive a travel time
distribution over RRPs of up to 1 year.

   - The core methodology is less computationally intensive than
microsimulation.

   - The core methodology can be directly calibrated on the basis of local
or regional capacity defaults to replicate recurring bottlenecks.

   - It considers local and regional weather defaults for the 101 largest
U.S. metropolitan areas on the basis of a 10-year average.

   - It encompasses a method for estimating incident and crash rates in the
absence of detailed local incident logs.

   - The method can be extended to evaluate ATDM strategies.

In addition, the strengths of the core methodology described in Chapter
10 apply to the reliability and strategy assessment methods presented here.


**Limitations of the Methodology**


Because the reliability method applies the freeway facilities core
methodology multiple times, it inherits the core methodology’s limitations.
These limitations were described in Chapter 10. For example, one limitation
of the core method is its use of 15-min analysis periods. Therefore, all event
durations (e.g., weather, incidents) used by the reliability method must be
expressed as integer numbers of 15-min analysis periods. The reliability
method has the following additional limitations:

   - The method assumes that the effect of two or more factors (weather
and incident) on speed or capacity is multiplicative. This assumption
has not been sufficiently tested empirically and may overstate the
influence of combined nonrecurring congestion effects.

   - Weather events with a small capacity reduction effect (<4%) are not
addressed. A given weather event (e.g., rain, snow) is always
assumed to occur at its mean duration value. Sun glare is not
accounted for.

   - The method assumes that incident occurrence and traffic demand are
independent of weather conditions, although all are indirectly tied to
each other through the specification of demand, incident, and weather
probabilities on a calendar basis. However, the analyst is able to
adjust incident frequencies by month on the basis of local data.

   - The method estimates incident occurrence as a function of segment
demand and month of the year. It does not consider potentially
elevated incident rates in segments with low demands. Some
segments may be overly prone to incidents due to poor visibility,
poor geometry, a short weaving segment, or other factors that are not
considered by the reliability method.

   - The method does not consider full facility closures in the scenarios.
In assigning incidents to the segments, at least one lane should
therefore remain open. The scenario generation methodology does not
assign incidents that result in full segment closure; it reassigns those
probabilities to other (less severe) incidents. This is also true for
work zones, where at least one travel lane has to remain open.

   - The travel time reliability analysis assumes similar effects of demand
variation and weather conditions on general purpose and managed


lanes, when a managed lane facility is included in the analysis.

   - Work zone events are only allowed to be modeled in general purpose
lanes; no managed lane work zone effects are considered.

   - The traffic demand adjustment assumes a proportional demand effect
across the entire facility, which means that all inputs and outputs
(across time and space in the base scenario) are increased or
decreased by the same factor.


**REQUIRED DATA AND SOURCES**

As a starting point, all of the input data normally needed in applying the
freeway facilities core methodology are required. These requirements are
given in Chapter 10. A base scenario is always required and is used to
describe base conditions (particularly demand and factors influencing
capacity and free-flow speed). The base scenario is intended to represent
average demand conditions (e.g., AADT) or the demand measured on a
specific day. This chapter’s methods factor these demands on the basis of
user-supplied or defaulted demand patterns to generate demands
representative of all other time periods during the RRP.

Additional data beyond those necessary for an HCM freeway facility
analysis are required for a reliability evaluation. Exhibit 11-10 lists the
general categories of data that are required by data type. Details are
provided in the following subsections.


**Exhibit 11-10: Required Input Data, Potential Data Sources, and Default Values for**
**Freeway Reliability Analysis**



**Data**
**Category** **Potential Data Source**



**Suggested**
**Default**
**Value**



Time User-defined study period, representative data of base Must be
periods scenario, and RRP provided
Demand Field data or modeling to generate day-of-week by month-of- Urban and
multipliers year demand factors rural defaults
provided in
Section 5
Weather Online database for probabilities of various intensities of rain, Defaults for
snow, cold, and low visibility by month 101 largest
U.S.


metropolitan
areas
provided in
Chapter 25
Incidents Field data estimates of frequencies of occurrence of shoulder Estimated
and lane closures per study period for each month, incident from
severity distribution, and average incident durations; segment
alternatively, crash rate and incident-to-crash ratio for the AADT and
facility, in combination with defaulted incident type probability lengths as
and duration data described in



Incidents Field data estimates of frequencies of occurrence of shoulder Estimated
and lane closures per study period for each month, incident from
severity distribution, and average incident durations; segment
alternatively, crash rate and incident-to-crash ratio for the AADT and
facility, in combination with defaulted incident type probability lengths as
and duration data described in

Chapter 25
Work User input on changes to base conditions and their schedule Must be
zones and provided
special
events



User input on changes to base conditions and their schedule Must be
provided



Nearest Select from the list of metropolitan areas provided in the Volume Must be
city 4 Technical Reference Library provided
when default
weather data
are used
Geometrics No details beyond core methodology needed. Obtained from Must be
road inventory or aerial photo provided
Traffic Demand multiplier represented in base dataset. Base scenario Must be
counts data from field data or modeling provided


As shown in Exhibit 11-10, most reliability-specific inputs can be
defaulted or are already required by the core methodology. Section 5,
Applications, provides default values that allow analysts in “data poor”
regions lacking detailed demand, weather, or incident data to apply this
chapter’s methods and obtain reasonable results. At the same time, the
method allows analysts in “data rich” regions to provide detailed local data
for these inputs when the most accurate results are desired.

Although default values are provided for many of the variables that affect
facility reliability (see Section 5, Applications), travel time reliability (as
measured by _TTI_ 80 or _TTI_ 95) can vary widely, depending on the
characteristics of a particular facility and the length of the study period.
Therefore, analysts are encouraged to use local values representative of
local demand, weather, and incident patterns whenever such data are
available. In addition, analysts must supply local values for work zones and
special events if they wish to account for these effects in a reliability
analysis. This subsection identifies potential sources of these data.


**Demand Patterns**

The best potential source of demand pattern data is a permanent traffic
recorder (PTR) located along the facility. Alternatively, an analyst may be
able to use data from a PTR located along a similar facility in the same
geographic area. Many state departments of transportation produce
compilations of data from their PTRs and provide demand adjustment factors
by time of day, day of week, and month of year by facility and area type. The
analyst is reminded that measured volumes are not necessarily reflective of
demands. Upstream bottlenecks may limit the volume reaching a PTR or
other observation point.


**Weather**

The National Climatic Data Center (NCDC) provides rainfall, snow, and
temperature statistics for thousands of locations through its website ( _6_ ) and
average precipitation rate data in the _Rainfall Frequency Atlas_ ( _7_ ). The more
detailed hourly weather data needed for a freeway facility analysis are
available from larger airport weather stations and can be obtained from the
NCDC website or other online sources [e.g., Weather Underground ( _8_ )].

A weather station that an agency has installed along the study facility may
also be able to provide the required data, if the agency stores and archives
the data collected by the station. A 10-year weather dataset is desirable for
capturing weather events that are rare but have a high impact.

Finally, analysts should consider the location of the facility relative to the
weather station. Elevation differences, proximity to large bodies of water,
and other factors that create microclimates may result in significant
differences in the probabilities of certain types of weather events (e.g., snow,
fog) on the facility and at the weather station.


**Incidents**

A significant level of effort is required to extract information about the
numbers and average durations of each incident type from the annual incident
logs maintained by roadway agencies, even in data-rich environments.
Furthermore, certain incident types—particularly shoulder incidents—can be
significantly underreported in incident logs ( _1_ ). Thus, the direct approach of


estimating incident probabilities is reserved for the rare cases where
incident logs are complete and accurate over the entire RRP. An alternative
approach is to estimate the facility incident rate from its predicted crash rate
and assume that the number of incidents in a given study period is Poisson
distributed ( _9_, _10_ ). Details of the process are described in Chapter 25,
Freeway Facilities: Supplemental.


**Work Zones**

A schedule of long-term work zones indicating the days and times when
the work zone will be in force and the portions of the roadway that will be
affected should be obtained from the roadway operating agency. Work zones
that vary in intensity (e.g., one lane closed on some days and two lanes
closed on others) or that affect different segments at different times will need
to be specified as two different work zones. When detailed traffic control
plans for each work zone are available, they should be consulted to
determine the starting and ending locations of lane closures, along with any
reductions in the posted speed. When detailed plans are not available, the
agency’s standard practices for work zone traffic control can be consulted to
determine the likely traffic control that would be implemented, given the
project’s characteristics.


**Special Events**

Special events are short-term events, such as major sporting events,
concerts, and festivals, that produce intense traffic demands on a facility for
limited periods. Special traffic control procedures may need to be
implemented to accommodate the traffic demands. The analyst should
identify whether any events that occur in or near the study area warrant
special treatment. If so, a schedule for the event (dates, starting times, typical
duration) should be obtained. Some types of events also have varying
intensities that will require separate treatment (e.g., a sold-out baseball game
compared with a lower-attendance midweek game). Recurring events may
have developed special traffic control procedures; if so, these plans should
be consulted to identify any changes required from base conditions. Each
combination of special event venue and event intensity to be included in the
analysis will need to be specified.


**METHODOLOGY OVERVIEW**

The methodologies for freeway reliability and freeway strategy
assessment are the second and third of the three parts of an evaluation
sequence starting with the evaluation of the freeway facility for a base
scenario. Part A: Core Freeway Facility Analysis (Single Study Period) was
presented in Chapter 10. Part B: Comprehensive Freeway Reliability
Analysis is the methodology presented in this section. Part C: Reliability
Strategy Assessment is presented in Section 4. It allows for the evaluation of
ATDM strategies.

Completion of the core methodology’s computational steps (Steps A-1
through A-17) is a prerequisite for conducting a reliability analysis (Steps B1 through B-13, depicted in Exhibit 11-11). Completion of a reliability
analysis is a prerequisite for an ATDM strategy assessment (Steps C-1
through C-9, presented in Section 4).


**Exhibit 11-11: Freeway Reliability Methodology Framework**


Note: - Steps shaded in gray are performed by the computational engine.


**COMPUTATIONAL STEPS**

This section describes the reliability methodology’s computational steps.
To simplify the presentation, the focus is on the function of and rationale for
each step. Chapter 25, Freeway Facilities: Supplemental, contains an
expanded version of this section that provides the supporting analytical
models and equations.


**Step B-1: Define RRP and Exclude Days**

In this step the analyst defines the duration of the RRP, which is typically
1 calendar year to encompass all day-to-day and month-to-month variability
in demand as well as all incident and weather patterns observed over the
calendar year. Periods shorter than 1 year can be selected for specific
analysis questions, in 1-day increments. For example, an analyst may be
interested in evaluating the reliability of a freeway only during the summer
tourist season or during the local construction season (excluding winter
months). In combination with the strategy assessment extensions described in
Section 4, an analyst may decide to evaluate a weather management program
and impacts of freeway service patrols only for the winter months. As
described earlier, selecting a shorter RRP will generally require more
replications of the scenario generation process. RRPs longer than 1 year are
not recommended, because all variability sources considered in the method
are captured in a 1-year duration.


In this step the analyst also decides which days of the week to include in
the analysis. A reliability analysis is typically performed for the 5 weekdays,
although weekends can be included if desired. Exhibit 11-9 provided
guidance on the number of replications recommended for a weekend-only
analysis. However, if a facility experiences significantly different
performance on weekdays and on weekends or if different weekday and
weekend driver populations (e.g., commuter versus recreational trips) are
known to exist, the mixing of weekdays and weekends in the same reliability
analysis is strongly discouraged.

In defining the RRP, the analyst may decide to exclude 1 or more days
from the analysis. If the analyst is interested in “typical” weekday
performance, the analyst may wish to exclude holidays (and high-demand
travel days before or after the holiday itself) from the analysis.

The reliability analysis works from a base scenario that is evaluated with
the freeway facility core method presented in Chapter 10. Because the
methodology adjusts seasonal and day-of-week demand patterns relative to
the base scenario, the specific date represented by the base scenario needs to
be defined. That is, the demand values contained in the base scenario should
correspond to a specific day–month combination of the year; these demands
are then adjusted by the reliability method for the other scenarios it
generates.

Alternatively, the analyst may choose to provide demands representative
of an “average day” in the base scenario on the basis of AADT values. In this
case, the analyst would then use demand multipliers in Step B-5 that are
calculated relative to that average day, rather than to the base scenario day. In
other words, Steps B-1 and B-5 need to be coordinated to ensure that the
correct demand multiplier factors are applied.


**Step B-2: Gather Reliability Inputs**

This step collects the additional inputs needed for conducting a
reliability analysis, including demand variability by day of week and month
of year, weather data, incident records, work zone data, and special events.
Some default values and quick estimation methods are provided to aid the
analyst; these are described in detail in Chapter 25. A list of required data
and potential data sources was presented above.


**Step B-3: Define or Refine Global Inputs**

In this step, the analyst has a chance to revise two global calibration
parameters for the analysis: facilitywide jam density and the queue discharge
capacity drop. While multiple bottlenecks (with different CAFs) can exist
along a facility, these two parameters are assumed to be global for the entire
facility. This step should be treated with care, since these two parameters
were previously defined and calibrated for the core facility analysis. While
these parameters provide additional calibration tools for reliability analysis,
having a well-calibrated base file is preferable, and changing global inputs
for reliability assessment is not generally recommended. In general, the
output of a reliability analysis is better calibrated by varying DAFs, CAFs,
SAFs, the number of lanes closed by incident types, and the underlying
scenario probabilities. A detailed reliability calibration methodology is
presented in Chapter 25.


**Step B-4: Define Number of Replications for Reliability Analysis**

In this step, the analyst specifies the number of replications used to
generate scenarios. The default number of replications for a 12-month RRP is
four, to ensure a sufficiently large sample of randomly generated weather and
incident events. The Spatial and Temporal Limits discussion earlier in this
section, along with Exhibit 11-9, provides guidance for modifying the number
of replications for shorter RRPs.

The goal of the hybrid scenario generation approach (with some
deterministic and some stochastic inputs) is to reduce the number of
scenarios from potentially several thousand to a few hundred representative
scenarios that capture the effects of all sources of nonrecurring congestion.
For most reliability applications, 240 scenarios (5 weekdays, 12 months, and
4 replications) are sufficient to capture the 1-year variability in performance.
However, the analyst may choose to include rarer scenarios (e.g., a 5- or 10year storm) to evaluate the impacts of very rare events. When an ATDM
strategy evaluation will also be conducted, a smaller number of scenarios is
recommended to allow for scenario-specific selection of ATDM strategies,
as discussed in Section 4.


**Step B-5: Define Demand Variability by Day and Month and**
**Assign to Scenarios**

This step defines demand multipliers by day of the week and by month of
the year on the basis of facility-specific data. The demand multiplier is
expressed relative to the base scenario demand date from the core freeway
facility defined in Step B-1. Alternatively, the analyst may select an average
demand day (estimated from AADTs) and express demand variability
relative to that day. The base scenario day does not need to be an average
day (i.e., it can have high or low demand relative to average conditions,
which is accounted for in this step) but should be free of special events or
nonrecurring sources of congestion.

Default values for urban and rural demand patterns are provided in
Section 5. They were developed from a national freeway demand dataset ( _2_ ).
However, facility-specific data are strongly preferred and are usually readily
obtainable from permanent traffic count stations or online sensor databases.


**Step B-6: Define Weather Probabilities and Impacts and Assign**
**to Scenarios**

This step defines the probabilities of occurrence of each of the HCM
weather types, along with corresponding CAFs, SAFs, and DAFs. They are
timewise probabilities. They represent the chance of occurrence of a weather
event at any instant in time and do not correspond to frequencies of weather
events. In other words, frequencies of weather events are converted to
probabilities on the basis of time of day and month of year. Default weather
type probabilities are provided for the 101 largest U.S. metropolitan areas in
the Volume 4 Technical Reference Library. CAF and SAF defaults are
provided in Section 5, but values developed from local data can be used
instead.

No default DAFs are available at this time, although extreme weather
events are generally understood to affect traffic demands. For example,
nighttime or early morning snowstorms are expected to reduce the demand
levels in the a.m. peak period, while multiday snow events are likely to
reduce both a.m. and p.m. peak demands. This effect also depends on
location (e.g., Boston versus Atlanta). Afternoon snowstorms may be less
likely to affect p.m. peak demand, since commuters may not have altered


their home-to-work trips that morning. Analysts are encouraged to develop
customized weather demand adjustment factors or apply judgment on the
basis of local conditions and experience.


**Step B-7: Define Incident Frequencies and Impacts and Assign**
**to Scenarios**

This step defines incident frequencies for each of the HCM incident
severity types, along with the corresponding CAFs, SAFs, and DAFs and the
number of lanes lost due to the incident. Default CAF and SAF values are
provided in Section 5, while DAFs will need to be user-defined. A quick
method for estimating incident frequencies on the basis of each segment’s
daily demand levels is provided in Chapter 25. However, facility-specific
data are preferable in specifying incident frequencies.

Chapter 25’s incident frequency estimation considers the total traffic
demand on a segment on the day represented by the base scenario to generate
incident frequencies for reliability analysis. Because different analysis
segments have different demand levels, the estimated incident rates will also
differ as a function of that demand. Accordingly, the scenario generation step
is more likely to generate more incidents on segments with higher demand,
which affects the overall reliability performance.

User-specified incident rates are especially important if an analyst is
aware of recurring monthly variations in incidents. If, for example, incidents
are more likely in winter months (despite potentially lower demands), the
analyst should adjust the incident rate defaults and calibrate for local
conditions.


**Step B-8: Define Short-Term Work Zone Events and**
**Adjustments**

This step defines the dates of any short-term work zone events, along
with the corresponding CAFs, SAFs, and DAFs and the number of lanes lost
due to the work zone. The phrase “short-term work zones” in this case refers
to scheduled or planned work zones that do not cover the entire RRP. For
example, if a work zone is in place for 1 or 2 months in a 1-year RRP, the
configuration should be entered here.


Long-term work zones, or those that cover the entire RRP, should be
evaluated as a stand-alone reliability analysis, with a base scenario modified
to reflect the work zone characteristics. One exception is a long-term work
zone that covers the entire RRP but that is divided into different stages or
configurations with varying CAFs, SAFs, or DAFs or different affected
segments. In that case, each stage can be accounted for separately and
sequentially in this step.

DAFs for short-term work zones are user-defined. A method for
estimating CAF and SAF values for work zones is provided in Section 4 of
Chapter 10.

Nonscheduled work zones, including very short (i.e., single-day)
activities (e.g., shoulder closure for landscaping work, lane closure for
pothole filling), are best addressed as a form of (random) incident in Step B7 rather than by explicitly defining their occurrence and location in this step.


**Step B-9: Generate Full Scenario List and Scenario Probabilities**

This step generates the listing of all scenarios for reliability analysis on
the basis of the inputs provided in the previous steps. The step is
automatically executed by the computational engine or other software tools.
The number of scenarios is a function of the user input in previous steps,
including the length of RRP (in months), the number of days generally
included in each week, the number of days specifically excluded, and the
number of replications. The scenario generation process is summarized here
and described in detail in Chapter 25, Freeway Facilities: Supplemental.

Each scenario will have a complete set of attributes defining the
characteristics of that scenario relative to the base scenario. Specifically,
each scenario will have a series of five matrices that define the demand
multipliers (from Step B-5), along with CAF, SAF, and DAF values and
adjustments to the number of lanes (Steps B-6 through B-8). The size of each
of the adjustment matrices will be equal to the number of analysis segments
times the number analysis periods contained within the base scenario. When
managed lanes are included in the analysis, the size of these matrices will
double to provide similar information for the managed lanes.

Whenever a scenario contains multiple adjustment effects due to weather,
incidents, or work zones, the methodology assumes that any two or more


CAFs, SAFs, or DAFs are multiplicative (i.e., independent). The number-oflanes adjustment factors are additive for incident and work zone events.

For example, with regard to the weather and incident combination in
Exhibit 11-5, the size of each adjustment matrix is 10 segments by 12
analysis periods. All 120 cells will be subject to demand multipliers from
Step B-5. In addition, a 45-min rain event in Analysis Periods 3 through 5
will result in CAF, SAF, and DAF adjustments for the entire 10-segment
facility during those analysis periods (30 cells). A two-lane closure incident
in Segment 8 in Analysis Periods 5 through 8 will reduce the number of lanes
in that segment for those four analysis periods. In addition, CAF, SAF, and
DAF adjustments are provided. The incident overlaps the rain event in one of
the analysis periods (Segment 8, Analysis Period 5), resulting in a
multiplicative effect of adjustments due to weather and incident. Finally, a
15-min shoulder-closure incident in Segment 3 in Analysis Period 11 results
in CAF, SAF, and possibly DAF adjustments.

If 240 scenarios are generated for the example in Exhibit 11-5, a total of
144,000 (5 adjustment matrices × 120 cells per matrix × 240 scenarios)
adjustment factors will be applied. The computational engine or other
software automatically performs the record keeping and estimation of these
factors.


**Step B-10: Perform Analysis for Each Scenario**

This step automatically processes each scenario in the computational
engine or other software. The adjustment matrices from Step B-9 are applied
sequentially to the base scenario, and the resulting scenarios are evaluated
individually with the Chapter 10 core methodology. The computational
engine or software produces the facilitywide performance measures for each
scenario.


**Step B-11: Compute Reliability Performance Measures**

This step generates a travel time distribution from the stored average
facility travel times by analysis period and scenario. It also computes a
variety of reliability performance measures from the results of all scenarios:

   - _TTI_ 95 (PTI),


   - _TTI_ 80,

   - _TTI_ 50,

   - _TTI_ mean,

   - Reliability rating,

   - Semi–standard deviation,

   - Standard deviation,

   - Failure or on-time percentage based on a target speed,

   - Policy index based on a target speed, and

   - Misery index.

These performance measures were defined in Section 2. Their
computation is automated by the computational engine or other software.
Additional details for computing reliability performance measures are
provided in Chapter 25.

The example facility shown in Exhibit 11-5 will generate 12 facility
travel times per scenario, one per analysis period. Multiplication by 240
scenarios will result in 2,880 facility travel time observations that define the
full travel time distribution. When these observations are sorted from highest
to lowest, the _TTI_ 95 is the travel time value ranked number 144 (0.05 ×
2,880) in the sorted list, while the _TTI_ 50 is the value ranked 1,440, and so on.


**Step B-12: Validate Against Field Data**

In this step, the reliability results are compared with field data, results
from another model, or expert judgment if no other data are available. If an
acceptable match is not obtained, the analysis returns to Step B-3 to make
calibration adjustments and then repeats the subsequent steps. Additional
details on criteria for calibrating and validating the facility are presented in
Chapter 25.


**Step B-13: Report Performance Measures**


This final step of the reliability assessment methodology reports the
facility’s reliability performance measures. Step B-13 concludes the
reliability analysis methodology. At this time, the analyst may choose to
continue to perform an ATDM evaluation, as described in Section 4. Note
that no level of service is defined for a reliability analysis. The analysis
instead presents various reliability performance measures, as well as the
resulting travel time distribution.


# **4. EXTENSIONS TO THE METHODOLOGY**

**ACTIVE TRAFFIC AND DEMAND MANAGEMENT**

ATDM is the dynamic management, control, and influence of travel
demand, traffic demand, and traffic flow on transportation facilities. Through
the use of tools and countermeasure strategies, traffic flow is managed and
traveler behavior is influenced in real time to achieve operational
objectives. The objectives include preventing or delaying breakdown
conditions, improving safety, promoting sustainable travel modes, reducing
emissions, and maximizing system efficiency.

This section provides an analysis framework, recommended measures of
effectiveness, and a methodology for evaluating the impacts of ATDM
strategies on freeway demand, capacity, and performance. Although this
section describes various ATDM “strategies” and “measures,” almost any
system management or operations strategy that is applied in a dynamic
manner can be considered active management.

The methodology presented here is primarily focused on traffic
management applications. In some cases, the operational strategies presented
here may be relatively static (e.g., fixed ramp-metering rates or pricing
schedules). The primary focus of ATDM analysis in the HCM is to provide
practitioners with practical, cost-effective methods for representing the
varied demand and capacity conditions that freeway facilities may be
expected to operate under. The method enables an analyst to apply a realistic
set of transportation management actions to respond to those conditions and
thus represent, in a macroscopic sense, the dynamic aspects of ATDM.

The ATDM analysis builds on the freeway reliability analysis
methodology, which accounts for freeway performance under different
demand, weather, incident, and work zone conditions. The ATDM extension
then superimposes one or more strategies on the completed reliability
analysis with the goal of improving reliability and other performance
measures. Often, the results of an ATDM strategy evaluation would be


compared with those of a more traditional capital improvement program that
adds physical capacity to the facility in question.


**ATDM Strategies and Plans**

ATDM strategies are evolving as technology advances. Typical ATDM
strategies can be classified according to their purpose and the manner in
which they are applied. Among them are the following:

   - Ramp-metering strategies,

   - Traveler information strategies,

   - Managed lane strategies, and

   - Speed harmonization strategies.

A more detailed discussion of ATDM strategies is provided in Chapter
37, ATDM: Supplemental. Specialized ATDM programs or plans may be
designed to address certain situations. For example, a _weather traffic_
_management plan_ may be developed to apply ATDM strategies during
adverse weather events. A _traffic incident management plan_ may apply
ATDM strategies specifically tailored to incidents. A _work zone_
_maintenance-of-traffic plan_ may apply ATDM strategies tailored to work
zones. _Employer-based demand management plans_ may apply major
employer–related ATDM strategies to address recurring congestion as well
as special weather and incident events.

The ATDM methodology distinguishes between five principal categories
of strategies that can affect facility operations:

1. _Demand management strategies_ that affect the entire scenario and all
segments and analysis periods contained within it when they are
invoked through a global increase or (more commonly) a reduction in
demand.

2. _Weather management strategies_ that influence performance only
during analysis periods when a severe weather event affects the
facility and apply equally to all segments. Weather management may
include driver information, weather-response strategies (e.g., snow
removal), and others.


3. _Incident management strategies_ that only affect the segment and
analysis periods when an incident is present. Incident management
may include freeway service patrols that result in reduced incident
clearance times, driver information, and others.

4. _Work zone management strategies_ that only affect the segment and
analysis periods when a work zone is present. Work zone
management may include driver information and other strategies.

5. _Special segment-specific strategies_ not covered in the previous
items, such as hard shoulder running and ramp metering. These
strategies specifically alter the capacity of one or more targeted
segments and are thus different from global demand management
strategies. For example, ramp metering will only affect the entry
traffic demand for merge and weave segments. Similarly, hard
shoulder running specifically increases capacity in a subset of
segments rather than the facility as a whole.

An ATDM plan is a combination of analyst-defined strategies.
Conceptually, each ATDM plan combines one or more ATDM strategies into
a package of system interventions available to a traffic management center or
operating agency. In the context of this methodology, there is no fundamental
difference between evaluating a single ATDM strategy and a combination of
strategies expressed as a plan. Similar to the reliability methodology
described in Section 3, the strategy or plan is ultimately translated into a
series of HCM inputs and adjustment factors to demand, capacity, and speed.

From a methodological perspective, only one set of inputs and
adjustments can be applied to each reliability scenario. Therefore, if multiple
strategies are to be evaluated, they need to be combined into an ATDM plan
and then applied to the scenario in question. For example, an incident ATDM
plan could include a variable message sign (a demand management strategy)
and traffic diversion (an incident management strategy) to avoid or alleviate
congestion. The two strategies affect the facility in different ways (since they
belong to two different categories) but are combined into a single plan for
analysis.


**Spatial and Temporal Limits**


The ATDM methodology is an extension of the freeway reliability
methodology and thus has the same spatial and temporal limits discussed in
Section 3.


**Limitations of the Methodology**

Several limitations apply to the ATDM extensions of this methodology:

   - If managed lanes are to be assessed as a strategy in an ATDM
analysis, they need to have been included in the base scenario used
for the core facility analysis. As described in Chapter 10, Freeway
Facilities Core Methodology, managed lanes can affect the
segmentation of the facility as well as the scenario generation
process. Thus, a “before-and-after” managed lane analysis requires
two core facilities, each with a separate reliability analysis.

   - This chapter focuses on numerical measures of performance;
however, much can be learned by examining graphical measures of
performance, such as the facility’s speed profile over time and over
the length of the facility. This approach can be particularly useful in
diagnosing the causes and extent of unreliable performance.

   - The ATDM analysis framework translates real-time dynamic control
systems into their HCM-equivalent average capacities and speeds for
15-min analysis periods, the smallest unit of time measurement
supported by the HCM. Therefore, some of the more dynamic aspects
of ATDM must be approximated in this analysis. Because the core
methodology for freeway facility analysis is deterministic, only the
average impacts of ATDM strategies on demand, speed, and capacity
are incorporated in this methodology.

   - ATDM is about controlling demand as well as capacity; however,
consistent with the rest of the HCM, this chapter focuses on the
capacity impacts of ATDM. Demand is an input to these procedures
that the analyst must determine with other tools. Demand variability
is considered where it influences total demand for the facility (such
as peaking within the peak period and variations between days of the
year). Demand changes are also considered in the methodology when
they are the result of direct controls imposed on the facility, such as
ramp metering and vehicle type restrictions (e.g., high-occupancy


vehicle lanes and truck lane restrictions). However, prediction of
how much additional traffic might be attracted to the facility with the
improved performance resulting from ATDM (sometimes called
“induced demand”) is not included in the chapter’s methodology.


**Strengths of the Methodology**

The following are strengths of the ATDM methodology:

   - The ability to target ATDM strategies to scenarios on the basis of
their operational characteristics.

   - The ability to compare ATDM strategies with traditional, capacitybased facility improvements (e.g., adding lanes).

   - The ability to contrast and compare different strategies or sets of
strategies in terms of their whole-year effects on the facility. In
combination with analyst-supplied cost estimates for the strategies,
the method supports a cost–benefit analysis of the strategies.

   - The ability to obtain before-and-after comparisons of the effect of
ATDM strategies quickly.

   - The ability to examine the whole-year effect of specific strategies that
may be seasonal (e.g., snow removal) and compare trade-offs with
other, nonseasonal strategies.


**Required Data and Sources**

The ATDM methodology requires as input the analyst-defined ATDM
strategy or a set of strategies combined into an ATDM plan. The method
requires the user to specify the impact of the selected strategies on demand,
capacity, free-flow speed, and number of lanes. The impact on demand,
capacity, and free-flow speed needs to be converted into matrices of average
adjustment factors (DAF, CAF, and SAF) affecting the base condition of the
freeway facility in each 15-min analysis period. Guidance and research on
the effectiveness of different ATDM strategies are limited.


**Adjustments to the Reliability Methodology**


The ATDM methodology builds on the reliability analysis described in
Section 3, which in turn builds on a calibrated core freeway facility analysis,
as described in Chapter 10. The scenarios used for reliability analysis
should be generated and calibrated to reflect the facility’s operational
conditions under different recurring and nonrecurring sources of congestion.
Once these steps are taken, the analyst can proceed with the ATDM analysis.
Exhibit 11-12 presents the additional nine steps that follow the reliability
analysis in performing an ATDM analysis.


**Exhibit 11-12: Freeway ATDM Strategy Evaluation Framework**


Note: - Steps shaded in gray are performed by the computational engine.


**Computational Steps**


_Step C-1: Limit Scenario List_

In this step, the analyst may elect to consider a limited number of
scenarios from the reliability analysis to enable a more targeted application
of ATDM strategies. The preceding reliability analysis typically results in
approximately 240 scenarios for a 1-year RRP. The analyst may apply one or
more “global” ATDM strategies equally to all scenarios. In this case, Step C1 is not necessary and the analysis can proceed.

However, specific ATDM strategies are often applied only to a subset of
(reliability) scenarios, to target a specific operational condition. For
example, incident management strategies are applied to scenarios with
incidents, and work zone management strategies are only applied to
scenarios with work zones. At this time, this process of assigning ATDM
strategies to scenarios must be carried out manually, since no research results
are available to automate the assignment of ATDM strategies to reliability
scenarios. To facilitate this process, the list of 240 or so reliability scenarios
can be limited to an ATDM subset.

Agencies may have their own algorithms for automating the ATDM
strategy assignment process. In that case, no reduction in the number of
scenarios is necessary. Similarly, an analyst may elect to assign ATDM
strategies manually to all 240 or so scenarios if time and resources permit.

However, in the standard HCM ATDM analysis, the analyst is
encouraged to select a subset of scenarios for evaluation. This subset may
reflect a certain condition that is targeted by the ATDM strategy in question
(e.g., inclement weather days to test a snow removal strategy) but should
always include other (nonweather) scenarios, to avoid overestimating the
effect of the strategy on the entire RRP. Statistical tests of how well the
reduced scenario list reflects the overall population are included in the
calibration step.


The framework for ATDM analysis allows the user to select any number
or set of reliability scenarios for ATDM or other strategy implementation.
However, to generate confidence in the resulting before-and-after
comparisons, the analyst should consider the following guidelines for
selecting scenarios:

   - As general guidance, it is recommended that the analyst select at least
10 scenarios for an ATDM reliability analysis, and preferably 30.
Selecting fewer than 10 scenarios may produce significant bias and
error in the analysis outputs when the impact on the full system
reliability is tested. An ATDM strategy can also be applied to a
single scenario in a “before-and-after” core facility analysis by using
the method in Chapter 10. Thus, the 10-scenario limit applies only to
a _reliability_ analysis evaluating before-and-after ATDM effects.

   - In comparing the effect of ATDM strategies on the entire set of
reliability scenarios, the selection must include broad spectrum
scenarios. One or more of these scenarios will need to be a “good
operational” scenario, in which the facility travel time is less than the
expected value, and one or more of the other scenarios should be a
“poor operational scenario.” This approach is important for accurate
prediction of the impact of the strategy on the full set of reliability
scenarios. In other words, the subset of scenarios selected for ATDM
analysis should be representative of the overall population of
scenarios from the reliability analysis and avoid bias toward overly
“good” or “poor” operating conditions. For example, picking a
scenario with no inclement weather or incidents has no impact on the
results of an “after” scenario when the selected strategy targets
improved incident response, but it will nevertheless improve
confidence in the comparison of reliability results.

   - The selection of ATDM scenarios is best related to the type of
strategies that the analyst intends to use. For example, if there is
interest in evaluating a set of work zone–related ATDM strategies,
the selected scenarios must have some work zone presence.

   - If the number of reliability scenarios required for characterizing a
certain event (i.e., work zone, weather, incident) is too low to meet
the 10-scenario threshold, the analyst should consider increasing the


number of replications used in the reliability scenario generation
process.


_Step C-2: Select Pool of ATDM Strategies_

This step allows the analyst to select which ATDM strategy or set of
strategies to include in the evaluation. A number of strategies are described
in Chapter 37, ATDM: Supplemental. Not every strategy or ATDM plan
needs to be applied to every scenario in the ATDM scenario list. For
example, a weather management plan may only apply to scenarios with
inclement weather, or a freeway service patrol strategy may only apply to
incident scenarios.


_Step C-3: Convert ATDM Information to Operational Inputs_

This step converts the ATDM strategy or plan into operational inputs
including DAFs, CAFs, SAFs, incident duration adjustments (if applicable),
and number-of-lanes adjustments. The HCM currently does not include
default values for ATDM strategies; thus, they must be input by the analyst on
the basis of judgment or local data. The reader is referred to Chapter 37,
ATDM: Supplemental, for additional information.


_Step C-4: Design ATDM Plans for the Facility and Assign to_
_Scenarios_

The analyst may elect to apply a strategy uniformly across all scenarios
but more commonly would match a specific strategy with a specific scenario
(e.g., weather management for snow events, service patrols for incidents).

As discussed earlier, multiple strategies can be combined into an ATDM
plan to result in a unique set of inputs (adjustment factors) applied to each
scenario. Only one set of these inputs can be applied to each reliability
scenario. If multiple strategies are combined, their respective DAFs, CAFs,
and SAFs are multiplied to produce a single DAF, CAF, and SAF for
application to the scenario, unless additional information is available on the
combined effect of pooled strategies.

The computational engine provides the user with a summary sortable
table of each scenario’s attributes (e.g., number of weather events, number of


incidents, maximum TTI) to assist the user in assigning an appropriate set of
ATDM strategies to the relevant scenarios.


_Step C-5: Process ATDM Scenarios_

This step evaluates each scenario by applying the core methodology from
Chapter 10. It is automatically performed by the computational engine or
other software implementation of the methodology.


_Step C-6: Compute Performance Measures_

This step calculates performance measures for the facility with the
ATDM strategies applied. Results are provided for each scenario, along with
an overall travel time (or distribution) using three comparison classes.

The first class compares the performance measure results for a _single_
_scenario_ before and after ATDM implementation. This class is useful as an
initial test and to verify the scenario assignments carried out in Step C-4. The
second class of output compares the _aggregated results_ for the combined but
limited set of scenarios defined in Step C-1 (e.g., the 30 scenarios selected
for ATDM implementation) before and after ATDM implementation. Finally,
the third class extrapolates the comparison to the entire travel time
distribution across the RRP on the basis of ATDM implementation in a
limited set of scenarios.


_Step C-7: Process Before-and-After Comparison_

This step conducts a before-and-after comparison of ATDM strategy
effectiveness by comparing the results of the reliability analysis with the
results of the ATDM analysis. The focus of this comparison is on the travel
time distribution before and after implementation of the ATDM strategy set.
Specific reliability performance measures, including _TTI_ mean and _TTI_ 95, can
be used for a high-level assessment of the improvement resulting from the
ATDM implementation. Generally, though, the overall travel time distribution
is of interest in making these comparisons.


_Step C-8: Validate Results_

In this step, the ATDM results are compared with field data (if
available), results from another model, or expert judgment. Field data on the


effects of ATDM strategies, especially on the reliability distribution, can be
difficult to obtain, and expert judgment may be more frequently applied in
this step. Additional details on facility calibration and validation criteria are
provided in Chapter 25. If an acceptable match is not obtained, the analysis
returns to Step C-3 to adjust the operational inputs.


_Step C-9: Report Performance Measures_

This final step of the ATDM assessment methodology reports the
facility’s reliability performance measures with the ATDM strategy or plan
applied. Additional performance measures may be generated for each
scenario.


# **5. APPLICATIONS**

**EXAMPLE PROBLEMS**

Section 11 of Chapter 25, Freeway Facilities: Supplemental, provides
four example problems that illustrate applications of the reliability and
strategy assessment methodologies to a freeway facility under various
operating conditions. Exhibit 11-13 lists these example problems.


**Exhibit 11-13: List of Example Problems**


**Example Problem** **Description** **Application**

1 Base reliability Operational analysis
2 Evaluation of geometric improvements Operational analysis
3 Evaluation of incident management Operational analysis
4 Planning-level reliability analysis Planning analysis


**EXAMPLE RESULTS**

This section presents the results of applying this chapter’s methodologies
in typical situations. Analysts can use the illustrative results presented in this
section to observe the sensitivity of output performance measures to various
inputs and to help evaluate whether their analysis results are reasonable. The
exhibits in this section are not intended to substitute for an actual analysis
and are deliberately provided in a format large enough to depict general
trends in the results but not large enough to pull out specific results.

Total travel time on a freeway facility is sensitive to a number of factors
including the prevailing free-flow speed, demand levels, segment capacity,
percent drop in queue discharge flow rate, demand-to-capacity ratio, weather
conditions, incidents, presence of work zones, and special events.
Consequently, these factors can influence travel time reliability on a freeway
facility.


Exhibit 11-14 shows four cumulative TTI distributions resulting from a
reliability analysis for the freeway facility given in Example Problem 1 in
Chapter 25. The “recurring congestion only” curve corresponds to a
reliability analysis assuming no inclement weather, incident events, or
scheduled work zones in the RRP. As expected, this curve yields consistently
lower (i.e., better) TTI values than do the other three TTI distributions. In
this case, _TTI_ 95 is 1.5.

The “recurring congestion + weather” curve corresponds to an analysis
in which inclement weather conditions are added to the variation in demand.
As expected, this addition slightly shifts the TTI distribution toward higher
TTI values without appreciably changing _TTI_ 95.

The “recurring congestion + incidents” curve captures variations in
demand level plus the occurrence of incidents during the RRP. As expected,
the inclusion of incidents increases TTI values for the entire distribution and,
consequently, results in a shift toward higher TTI values in the curve. In this
case, _TTI_ 95 increases to about 1.8, representing a 20% increase above the
base recurring-congestion case.

Finally, the “recurring congestion + weather + incidents” curve
corresponds to an RRP that includes variations in the demand level,
inclement weather events, and incidents. This curve models scenarios that
combine inclement weather events, incidents, and high demand values.
Therefore, the resulting TTI curve has higher TTI values than the other three
curves, although again _TTI_ 95 does not appreciably increase compared with
the “recurring congestion + incidents” case.


**Exhibit 11-14: Illustrative Effects of Different Nonrecurring Sources of Congestion on**
**the TTI Distribution**


Note: Based on Example Problem 1 from Chapter 25, using default weather data for
Raleigh, North Carolina, and a facilitywide incident rate of 1,050 incidents per 100
million VMT.


As shown above, the inclusion of inclement weather events in the RRP
shifts the TTI distribution toward higher TTI values. Exhibit 11-15 depicts
the TTI probability distribution function obtained with different weather
conditions (in this case, in a city with a milder climate). Bars with a dotted
pattern indicate a reliability analysis that is performed under the assumption
of a 10% chance of heavy snow in December, January, and February. Dark
bars correspond to an otherwise identical analysis performed under the
assumption of zero snow probability in those 3 months. The exhibit shows
that the higher heavy snow probability yielded a lower percentage of TTI
values in the 1 to 1.05 range. A lower snow probability resulted in a lower
percentage of higher TTI values.


**Exhibit 11-15: Illustrative Effects of Inclement Weather Events on the TTI Distribution**


Note: Based on Example Problem 1 from Chapter 25, with a facilitywide incident rate of
1,050 incidents per 100 million VMT and heavy snow probabilities of 0% and 10%.


Exhibit 11-16 illustrates the effects of incident frequency on travel time
reliability. The dotted curve corresponds to the travel time distribution
assuming 350 incidents per 100 million VMT. Increasing the rate from 350 to
700 incidents per 100 million VMT (dashed line) results in a shift in the TTI
distribution toward a higher value. This is expected, since a greater number
of scenarios are affected by incidents in this case. Increasing the rate from
700 to 1,050 incidents per million VMT (solid line) yields a further
rightward shift in the distribution, as expected.


**Exhibit 11-16: Illustrative Effects of Incident Rates on the TTI Distribution**


Note: Based on Example Problem 1 of Chapter 25, using default weather data for Raleigh,
North Carolina, and facilitywide incident rates of 350, 700, and 1,050 incidents per
100 million VMT.


The final example depicts the impacts of an ATDM strategy on travel
time reliability. Exhibit 11-17 shows two TTI distributions. The first
distribution is a base case (Example Problem 1 in Chapter 25), while the
second is from a case where a hard shoulder running strategy is applied to
the facility. As shown in the exhibit, allowing vehicles to use the shoulder
shifts the TTI distribution toward lower TTI values. This trend occurs
because hard shoulder running increases the capacity of the freeway facility
and, as a result, travel time is consistently reduced.


**Exhibit 11-17: Effect of Activating Hard Shoulder Running ATDM Strategy**


Note: Based on Example Problem 1 in Chapter 25, Raleigh, North Carolina, weather
conditions, and facilitywide incident rate of 1,050 incidents per 100 million VMT.


**DEFAULT VALUES**

This section provides default values for much of the input data used by
this chapter’s reliability methodologies. Agencies are encouraged, when
possible, to develop local default values on the basis of field measurements
of facilities in their jurisdiction. Local defaults provide a better means of
ensuring accuracy in analysis results. Facility-specific values provide the
best means of ensuring an adequate representation of local and regional
conditions. In the absence of local data, this section’s default values can be
used when the analyst believes that the values are reasonable for the facility
to which they are applied.


**Traffic Demand Variability**

Exhibit 11-18 and Exhibit 11-19 present default demand ratios by day of
week and month of year for urban and rural freeway facilities, respectively.
The ratios were derived from a national freeway dataset developed by
Strategic Highway Research Program 2 Project L03 ( _2_ ). All ratios reflect
demand relative to a Monday in January. Where possible, analysts should


obtain local or regional estimates of demand variability to account for
facility-specific and seasonal trends on the subject facility.


**Exhibit 11-18: Default Urban Freeway Demand Ratios (ADT/Mondays in January)**







|Month|Day of Week<br>Monday Tuesday Wednesday Thursday Friday Saturday Sunday|
|---|---|
|January<br>February<br>March<br>April<br>May<br>June<br>July<br>August<br>September<br>October<br>November<br>December|1.00<br>1.00<br>1.02<br>1.05<br>1.17<br>1.01<br>0.89<br>1.03<br>1.03<br>1.05<br>1.08<br>1.21<br>1.04<br>0.92<br>1.12<br>1.12<br>1.14<br>1.18<br>1.31<br>1.13<br>0.99<br>1.19<br>1.19<br>1.21<br>1.25<br>1.39<br>1.20<br>1.05<br>1.18<br>1.18<br>1.21<br>1.24<br>1.39<br>1.20<br>1.05<br>1.24<br>1.24<br>1.27<br>1.31<br>1.46<br>1.26<br>1.10<br>1.38<br>1.38<br>1.41<br>1.45<br>1.62<br>1.39<br>1.22<br>1.26<br>1.26<br>1.28<br>1.32<br>1.47<br>1.27<br>1.12<br>1.29<br>1.29<br>1.32<br>1.36<br>1.52<br>1.31<br>1.15<br>1.21<br>1.21<br>1.24<br>1.27<br>1.42<br>1.22<br>1.07<br>1.21<br>1.21<br>1.24<br>1.27<br>1.42<br>1.22<br>1.07<br>1.19<br>1.19<br>1.21<br>1.25<br>1.40<br>1.20<br>1.06|


Source: Derived from data presented by Cambridge Systematics et al. ( _2_ ).
Note: Ratios represent demand relative to a Monday in January.


**Exhibit 11-19: Default Rural Freeway Demand Ratios (ADT/Mondays in January)**







|Month|Day of Week<br>Monday Tuesday Wednesday Thursday Friday Saturday Sunday|
|---|---|
|January<br>February<br>March<br>April<br>May<br>June<br>July<br>August<br>September<br>October<br>November<br>December|1.00<br>0.96<br>0.98<br>1.03<br>1.22<br>1.11<br>1.06<br>1.11<br>1.06<br>1.09<br>1.14<br>1.35<br>1.23<br>1.18<br>1.24<br>1.19<br>1.21<br>1.28<br>1.51<br>1.37<br>1.32<br>1.33<br>1.27<br>1.30<br>1.37<br>1.62<br>1.47<br>1.41<br>1.46<br>1.39<br>1.42<br>1.50<br>1.78<br>1.61<br>1.55<br>1.48<br>1.42<br>1.45<br>1.53<br>1.81<br>1.63<br>1.57<br>1.66<br>1.59<br>1.63<br>1.72<br>2.03<br>1.84<br>1.77<br>1.52<br>1.46<br>1.49<br>1.57<br>1.86<br>1.68<br>1.62<br>1.46<br>1.39<br>1.42<br>1.50<br>1.78<br>1.61<br>1.55<br>1.33<br>1.28<br>1.31<br>1.38<br>1.63<br>1.47<br>1.42<br>1.30<br>1.25<br>1.28<br>1.35<br>1.59<br>1.44<br>1.39<br>1.17<br>1.12<br>1.14<br>1.20<br>1.43<br>1.29<br>1.24|


Source: Derived from data presented by Cambridge Systematics et al. ( _2_ ).
Note: Ratios represent demand relative to a Monday in January.


**Weather Event Probabilities**

Weather event probabilities by month of each weather event for the
largest U.S. metropolitan areas are provided as resource material in the
Technical Reference Library in online HCM Volume 4. Average durations of
each severe weather event type are also provided for these metropolitan
areas.


**Weather Capacity and Speed Adjustment Factors**

Exhibit 11-20 and Exhibit 11-21 provide default CAFs and SAFs,
respectively, by weather type and facility free-flow speed. Note that the
changes in CAFs and SAFs for decreasing visibility shown in the exhibit may
be counterintuitive, since they are based on a single site.

The SAF is applied to the base free-flow speed, and the CAF is applied
to the base capacity, both of which are calculated in the respective
methodological chapters for the various freeway segment types. Both may
also have been adjusted in the process of calibrating the core facility in
Chapter 10. The adjustment factors below should be applied in addition to
any prior CAF and SAF calibration.


**Exhibit 11-20: Default CAFs by Weather Condition**







|Weather Type|Weather Event Definition|Capacity Adjustment Factors<br>55 60 65 70 75<br>mi/h mi/h mi/h mi/h mi/h|
|---|---|---|
|Medium rain<br>Heavy rain|>0.10–0.25 in./h<br>>0.25 in./h|0.94<br>0.93<br>0.92<br>0.91<br>0.90<br>0.89<br>0.88<br>0.86<br>0.84<br>0.82|
|Light snow<br>Light–medium snow<br>Medium–heavy snow<br>Heavy snow|>0.00–0.05 in./h<br>>0.05–0.10 in./h<br>>0.10–0.50 in./h<br>>0.50 in./h|0.97<br>0.96<br>0.96<br>0.95<br>0.95<br>0.95<br>0.94<br>0.92<br>0.90<br>0.88<br>0.93<br>0.91<br>0.90<br>0.88<br>0.87<br>0.80<br>0.78<br>0.76<br>0.74<br>0.72|
|Severe cold|<–4°F|0.93<br>0.92<br>0.92<br>0.91<br>0.90|
|Low visibility<br>Very low visibility<br>Minimal visibility|0.50–0.99 mi<br>0.25–0.49 mi<br><0.25 mi|0.90<br>0.90<br>0.90<br>0.90<br>0.90<br>0.88<br>0.88<br>0.88<br>0.88<br>0.88<br>0.90<br>0.90<br>0.90<br>0.90<br>0.90|
|Non–severe weather|All conditions not listed above|1.00<br>1.00<br>1.00<br>1.00<br>1.00|


Source: Zegeer et al. ( _1_ ).
Note: Speeds given in column heads are free-flow speeds.


**Exhibit 11-21: Default SAFs by Weather Condition**





|Weather Type|Weather Event Definition|Speed Adjustment Factors<br>55 60 65 70 75<br>mi/h mi/h mi/h mi/h mi/h|
|---|---|---|
|Medium rain<br>Heavy rain|>0.10–0.25 in./h<br>>0.25 in./h|0.96<br>0.95<br>0.94<br>0.93<br>0.93<br>0.94<br>0.93<br>0.93<br>0.92<br>0.91|
|Light snow<br>Light–medium snow<br>Medium–heavy snow<br>Heavy snow|>0.00–0.05 in./h<br>>0.05–0.10 in./h<br>>0.10–0.50 in./h<br>>0.50 in./h|0.94<br>0.92<br>0.89<br>0.87<br>0.84<br>0.92<br>0.90<br>0.88<br>0.86<br>0.83<br>0.90<br>0.88<br>0.86<br>0.84<br>0.82<br>0.88<br>0.86<br>0.85<br>0.83<br>0.81|
|Severe cold|<–4°F|0.95<br>0.95<br>0.94<br>0.93<br>0.92|
|Low visibility<br>Very low visibility<br>Minimal visibility|0.50–0.99 mi<br>0.25–0.49 mi<br><0.25 mi|0.96<br>0.95<br>0.94<br>0.94<br>0.93<br>0.95<br>0.94<br>0.93<br>0.92<br>0.91<br>0.95<br>0.94<br>0.93<br>0.92<br>0.91|
|Non–severe weather|All conditions not listed above|1.00<br>1.00<br>1.00<br>1.00<br>1.00|


Source: Zegeer et al. ( _1_ ).
Note: Speeds given in column heads are free-flow speeds.


**Incident Probabilities and Durations**

Exhibit 11-22 provides mean distributions of freeway incidents by
severity and default incident duration parameters by incident type.


**Exhibit 11-22: Default Freeway Incident Severity Distribution and Duration Parameters**
**(min)**







|Parameter|Incident Severity Type<br>Shoulder 1 Lane 2 Lanes 3 Lanes 4+ Lanes<br>Closed Closed Closed Closed Closed|
|---|---|
|Distribution<br>(%)<br>Duration<br>(mean)<br>Duration (std.<br>dev.)<br>Duration (min.)<br>Duration<br>(max.)|75.4<br>19.6<br>3.1<br>1.9<br>0<br>34<br>34.6<br>53.6<br>67.9<br>67.9<br>15.1<br>13.8<br>13.9<br>21.9<br>21.9<br>8.7<br>16<br>30.5<br>36<br>36<br>58<br>58.2<br>66.9<br>93.3<br>93.3|


Source: Zegeer et al. ( _1_ ).
Notes: std. dev. = standard deviation; min. = minimum; max. = maximum.


**Incident Capacity Adjustment Factors**

Exhibit 11-23 shows the default CAFs associated with each incident
severity. The values shown in the exhibit reflect the _remaining relative_
_capacity per open lane_ . For example, a two-lane closure incident on a sixlane directional facility (underscored) results in a loss of two full-lane
capacities, in addition to maintaining only 75% of the remaining four open
lanes’ capacities. The result is that only three lanes worth (50%) of the
facility’s original six-lane capacity is maintained. No information is
available on the effect of incidents on free-flow speed, so this effect is not
accounted for at this time.


**Exhibit 11-23: CAFs by Incident Type and Number of Directional Lanes on the Facility**


**Directional** **No** **Shoulder** **1 Lane** **2 Lanes** **3 Lanes** **4 Lanes**
**Lanes** **Incident** **Closed** **Closed** **Closed** **Closed** **Closed**

2 1.00 0.81 0.70 N/A N/A N/A
3 1.00 0.83 0.74 0.51 N/A N/A
4 1.00 0.85 0.77 0.50 0.52 N/A
5 1.00 0.87 0.81 0.67 0.50 0.50
6 1.00 0.89 0.85 0.75 0.52 0.52
7 1.00 0.91 0.88 0.80 0.63 0.63
8 1.00 0.93 0.89 0.84 0.66 0.66


Source: Zegeer et al. ( _1_ ).
Notes: N/A = not applicable — the number of lanes closed equals or exceeds the number of
directional lanes.


The methodology does not permit all directional lanes of a facility to be
closed.


**PLANNING, PRELIMINARY ENGINEERING, AND DESIGN**
**ANALYSIS**

A facility’s average travel time will vary from hour to hour, day to day,
and season to season, depending on fluctuations in demand, weather,
incidents, and work zones. Reliability measures characterize this distribution
of travel times for a selected period of a year meaningful to the analyst, the
agency’s objectives, and the general public.


Estimating performance measures requiring complex calculations, such
as the reliability distribution described in this chapter, can be challenging in
a planning context. However, two options exist for applying this chapter’s
reliability methodology in a planning context:

1. Application of HCM methods using default values and

2. Simplified percentile estimation method.

Both methods are introduced below and are described further in the
_Planning and Preliminary Engineering Applications Guide to the HCM,_
available in the Technical Reference Library in the online HCM Volume 4.


**HCM Method Using Default Values**

This chapter’s method for estimating travel time reliability can, to some
extent, be automated through the use of default values. Automating the
generation of inputs, along with applying the method in a computational
engine or software, allows reliability performance to be estimated with
minimal input needs, which may make the process suitable for application in
a planning context. Exhibit 11-24 lists the required input data and describes
where default values are provided.


**Exhibit 11-24: Input Data Needs for HCM Planning Reliability Analysis of Freeways**


**Data**
**Category** **Description** **Data Source**

Time Analysis period, study period, reliability Must be selected by the
periods reporting period analyst
Demand Default values provided in
Day-of-week by month-of-year demand factors
patterns Chapter 25

Probabilities of various intensities of rain, snow, Data sources and default
Weather
cold, and low visibility by month values provided in Chapter 25



Crash rate and incident-to-crash ratio for the
Incidents facility, in combination with defaulted incident
type probability and duration data



Crash rate and incident-to-crash ratio for the Crash rate must be provided;
Incidents facility, in combination with defaulted incident default values available in
type probability and duration data Chapter 25 for other data

Work
zones and Must be specified when

Changes to base conditions and schedule

special relevant to the analysis
events



Must be specified when
Changes to base conditions and schedule
relevant to the analysis



Nearest Required to apply weather
City with airport weather station
city defaults
Traffic Demand multiplier for demand represented in Must be provided


counts base dataset


**Simplified Method**

The equations in this section can be used to estimate specific TTI
percentiles as an approximation of freeway facility reliability ( _11, 12_ ). This
method does not specify the full reliability distribution, nor is it customized
to a specific facility’s geometry or operating characteristics.

First, the mean annual travel time index, including incident effects, is
computed:


**Equation 11-1:**


where

_TTI_ mean = average annual mean travel time index (unitless);

_FFS_ = free-flow speed (mi/h);

_RDR_ = recurring delay rate (h/mi), from Equation 11-2; and

_IDR_ = incident delay rate (h/mi), from Equation 11-3.


**Equation 11-2:**


**Equation 11-3:**


where

_S_ = peak-hour speed (mi/h),


_N_ = number of lanes in one direction ( _N_ = 2 to 4), and

_X_ = peak hour volume-to-capacity ratio (decimal).


Equation 11-3 is valid only for _X_ ≤1.00 and _N_ = 2, 3, or 4. Values of _X_
greater than 1.00 should be capped at 1.00, and values of _N_ greater than 4
should be capped at 4, for use in Equation 11-3.

The 95th percentile travel time index ( _TTI_ 95) and percent of trips
traveling under 45 mi/h ( _PT_ 45) can be computed from the average annual TTI
according to the following equations.


**Equation 11-4:**


**Equation 11-5:**


where

_TTI_ 95 = 95th percentile TTI (unitless),

_TTI_ mean = average annual mean travel time index (unitless), and

_PT_ 45 = percent of trips that occur at speeds less than 45 mi/h
(decimal).


**USE OF ALTERNATIVE TOOLS**

General guidance for the use of alternative traffic analysis tools for
capacity and level-of-service analysis is provided in Chapter 6, HCM and
Alternative Analysis Tools. This section contains specific guidance for
applying alternative tools to the analysis of freeway facilities. Additional
information on this topic may be found in Chapter 25, Freeway Facilities:
Supplemental.


In some cases, a finer temporal sensitivity to dynamic changes in the
system will be required for a reliability analysis than can be provided by the
typical 15-min analysis period used by HCM methods. This situation may
occur in evaluating traffic-responsive signal timing, traffic adaptive control,
dynamic ramp metering, dynamic congestion pricing, or measures affecting
the prevalence or duration of incidents with less than 10-min durations.
There may also be scenarios and configurations that the HCM cannot
address, such as complex merging and diverging freeway sections.

For such situations, this chapter’s conceptual framework for evaluating
travel time reliability can be applied to alternative analysis tools. The same
conceptual approach of generating scenarios, assigning scenario
probabilities, evaluating scenario performance, and summarizing the results
applies when alternative analysis tools, such as microsimulation, are used to
estimate the reliability effects of operations improvements.

Before embarking on the use of alternative tools for reliability analysis,
the analyst should consider the much greater analytical demands imposed by
a reliability analysis following this chapter’s conceptual analysis framework.
Thousands of scenarios may need to be analyzed with the alternative tool in
addition to the number of replications per scenario required by the tool itself
to establish average conditions. Extracting and summarizing the results from
numerous applications of the alternative tool may be a significant task.

If a microscopic simulation tool is used, some portions of this chapter’s
analysis framework that were fit to the HCM’s 15-min analysis periods and
tailored to the HCM’s speed–flow curves will no longer be needed:

   - Scenarios may be defined differently from and may be of longer or
shorter duration than those used in HCM analysis.

   - Incident start times and durations will no longer need to be rounded
to the nearest 15-min analysis period.

   - Weather start times and durations will no longer need to be rounded
to the nearest 15-min analysis period.

   - Demand will no longer need to be held constant for the duration of
the 15-min analysis period.

   - The peak hour factors used to identify the peak 15-min flow rate
within the hour will no longer be applied. They will be replaced with


the analysis tool’s built-in randomization process.

   - This chapter’s recommended freeway capacity adjustment factors,
along with the free-flow speed adjustment factors for weather events
and incidents, will have to be converted by the analyst to the
microsimulation model equivalents: desired speed distribution and
desired headway distribution. Acceleration and deceleration rates
will also be affected for some weather events.

   - This chapter’s recommended freeway speed–flow curves for weather
events and incidents will be replaced with adjustments to the model’s
car-following parameters, such as desired free-flow speed, saturation
headway, and start-up lost time. Unlike incidents, which the tool’s
car-following logic can address, weather is modeled by adjusting the
car-following parameters through weather adjustment factors before
the scenarios are run. Application guidance and typical factors are
provided in FHWA’s _Traffic Analysis Toolbox_ ( _13_ ).

If a less disaggregate tool is used (e.g., mesoscopic simulation analysis
tool, dynamic traffic assignment tool, demand forecasting tool), many of this
chapter’s adaptations of the conceptual analysis framework to the HCM may
still be appropriate or may need to be aggregated further. The analyst should
consult the appropriate tool documentation and determine what further
adaptations of the conceptual analysis framework might be required to apply
the alternative tool to reliability analysis.


# **6. REFERENCES**

1. Zegeer, J., J. Bonneson, R. Dowling, P. Ryus, M. Vandehey, W.
Kittelson, N. Rouphail, B. Schroeder, A. Hajbabaie, B. Aghdashi, T.
Chase, S. Sajjadi, R. Margiotta, and L. Elefteriadou. _Incorporating_
_Travel Time Reliability into the_ Highway Capacity Manual. SHRP 2
Report S2-L08-RW-1. Transportation Research Board of the National
Academies, Washington, D.C., 2014.

2. Cambridge Systematics, Inc., Texas A&M Transportation Institute,
University of Washington, Dowling Associates, Street Smarts, H.
Levinson, and H. Rakha. _Analytical Procedures for Determining the_
_Impacts of Reliability Mitigation Strategies._ SHRP 2 Report S2-L03RR-1. Transportation Research Board of the National Academies,
Washington, D.C., 2013.

3. Yeom, C., A. Hajbabaie, B. J. Schroeder, C. Vaughan, X. Xuan, and N.
M. Rouphail. Innovative Work Zone Capacity Models from Nationwide
Field and Archival Sources. In _Transportation Research Record:_
_Journal of the Transportation Research Board, No. 2485_,
Transportation Research Board, Washington, D.C., 2015, pp. 51–60.

4. Hajbabaie, A., C. Yeom, N. M. Rouphail, W. Rasdorf, and B. J.
Schroeder. Freeway Work Zone Free-Flow Speed Prediction from
Multi-State Sensor Data. Presented at 94th Annual Meeting of the
Transportation Research Board, Washington, D.C., 2015.

5. Park, B., T. K. Jones, and S. O. Griffin. _Traffic Analysis Toolbox_
_Volume XI: Weather and Traffic Analysis, Modeling and Simulation_ .
Report FHWA-JPO-11-19. Federal Highway Administration,
Washington, D.C., Dec. 2010.

6. _Comparative Climatic Data for the United States Through 2010_ .
National Climatic Data Center, National Oceanic and Atmospheric
Administration, Asheville, N.C., 2011. http://www.ncdc.noaa.gov.
Accessed Sept. 21, 2011.


7. _Rainfall Frequency Atlas of the U.S.: Rainfall Event Statistics_ .
National Climatic Data Center, National Oceanic and Atmospheric
Administration, Asheville, N.C., 2011.
http://www.ncdc.noaa.gov/oa/documentlibrary/rainfall.html. Accessed
Sept. 21, 2011.

8. Weather Underground. _Weather History._

http://www.wunderground.com/history/. Accessed April 2012.

9. Khattak, A., and N. Rouphail. _Incident Management Assistance_
_Patrols: Assessment of Investment Benefits and Costs._ Report
FHWA/NC/2005-02. North Carolina Department of Transportation,
Raleigh, 2005.

10. Skabardonis, A., K. F. Petty, R. L. Bertini, P. P. Varaiya, H. Noeimi, and
D. Rydzewski. I-880 Field Experiment: Analysis of Incident Data. In
_Transportation Research Record 1603,_ Transportation Research Board,
National Research Council, Washington, D.C., 1997, pp. 72–79.

11. Economic Development Research Group, Inc.; Cambridge Systematics,
Inc.; ICF International; Texas A&M Transportation Institute; and Weris,
Inc. _Development of Tools for Assessing Wider Economic Benefits of_
_Transportation._ SHRP 2 Report S2-C11-RW-1. Transportation
Research Board of the National Academies, Washington, D.C., 2014.

12. Cambridge Systematics, Inc. _IDAS User’s Manual,_ Table B.2.14.

http://idas.camsys.com/documentation.htm. Accessed Aug. 14, 2014.

13. Park, B., T. K. Jones, and S. O. Griffin. _Traffic Analysis Toolbox_
_Volume XI: Weather and Traffic Analysis, Modeling and Simulation_ .
Report FHWA-JPO-11-19. Federal Highway Administration,
Washington, D.C., Dec. 2010.


