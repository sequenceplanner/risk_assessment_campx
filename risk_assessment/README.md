# Action based risk assessment support

## Idea (defning risks and action outcomes with Behavior Trees)

## Idea (FMEA + STPA):
Behavior Trees (BTs) can serve as a useful visual tool for modeling behavior at a high level and assist in risk assessment. The concept is to break down individual actions (or operations on our case), assess the associated risks for each operation (similar to STPA), and evaluate the potential severity and likelihood of these risks. Each operations will have a list of risks (and/or potential error outcomes), where each risk has an assigned severity, probability, and detectability (similar to FMEA). Once risks are graded by severity, probability, and detectability, automated planners can use this data to simulate multiple scenarios. This will allow us to obtain concrete numbers for the likelihood and severity of different risky sequences of events. These results can be used to determine whether specific actions are needed to mitigate risks, and if so, how to effectively mitigate them.

## Background

### RPN & FMEA: Failure Mode and Effects Analysis

Has 3 parts: 
1.  Severity: Denotes the seriousness of the problem if it happens, with a focus on the consequences. The higher the number, the greater the severity.
2.  Occurrence: Denotes how likely the issue is to occur. To determine the rate of occurrence, you’ll want to look at all the potential causes of a failure and the
chance that those causes will occur. The higher the number, the greater the probability of occurrence.
3.  Detection: Denotes how easy or difficult it is to identify the problem. A higher rating means an issue is less likely to be detected either by engineers during the test phases of product development or by customers after product release. Therefore, the higher the number, the less likely the failure is detected.

This can be used by RPN (Risk Priority Number). RPN is a widely adopted method for risk assessment, and is used often in Design FMEAs (DFMEAs), Process FMEAs (PFMEAs), and other FMEA types. RPN is calculated as Severity * Occurrence * Detection. Using a 1 to 10 scale for each factor results in RPN values ranging from 1 to 1000.

### STPA: System-Theoretic Process Analysis

Has 4 parts:
1. Purpose: Defining the purpose of the analysis is the first step with any analysis method. What kinds of losses
will the analysis aim to prevent? 
2. Model: The second step is to build a model of the system called a control structure. A control structure
captures functional relationships and interactions by modeling the system as a set of feedback control
loops.
3. Identify unsafe control actions: The third step is to analyze control actions in the control structure to examine how they could lead to
the losses defined in the first step. These unsafe control actions are used to create functional requirements and constraints for the system. This step also does not change regardless of whether STPA is being applied to safety, security, privacy, or other properties.
4. Reasons: The fourth step identifies the reasons why unsafe control might occur in the system.

