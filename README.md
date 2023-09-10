License: MIT-0

 # POPI Pallet

 Proof of positive interaction. A collectivity that contributes toward a common goal.
 There are different kind of things that may be built, each thing will go through
 a set of lavoration steps, or to better call, interactions, registered on chain, as a proof of an increment that happened off-chain. This is in simple terms another user that verifies positively that your work contributed to move a product from the lavoration step n to the n+1.

Lets make a concrete example:
 a software company that is building an ecommerce website. They are following a
 kanban or scrum agile process reflected in a typical board with those columns:

 NEW	 | TODO | IN PROGRESS | CODE REVIEW | QA | READY TO DEPLOY | DEPLOYED | QA PRODUCTION | DONE
        Task1
 Task2
                              Task3
 ....

 We have tasks, task can be of 3 different kinds:
 1. UI = design the UI of a functionality
 2. FRONTEND DEV = i.e. coding in React.js
 3. SERVER DEV = i.e. implementing an API service consumed by the frontend
 4. BI = someone who analyses the user journey
 5. QA  = Someone who validates something built and creates a Bug Ticket
 6. PO/Product Manager = creator of tasks.
 7. PM/Scrum Master = Manages the Flow of Scrum / Kanban
  

 Every task must follow all the steps from NEW to DONE. The progress can never be done by the
 same person who worked on this specific task in this specific column.

 Task: change a color of a text "Click here to sign in" from RED To Yellow.

 This task is created by UI. The task goes inmediately to the NEW column.
 PO moves this to TODO.

 A developer PULLS this task, means: moves this task from TODO into IN PROGRESS>
 Once He/She is done, set the task into "COMPLETED the current step". So small detail:
 yes, someone else must move the task, but the person in this step must say "MY WORK IS READY TO
 BE PULLED" So the board is simplified but actually it has the double of steps:

 NEW| READY TO BE PULLED TO TODO | TODO| READY FOR INPROGRESS | INPROGRESS | READY FOR REVIEW |
 REVIEW |

 three cases connected to the previous example:
 1. UI creates the task, this task must be well described, contain the color code corresponding
    to Yellow. When she thinks the task is ready to be pulled by A PO, set the task as "READY TO BE PULLED"

 2. When a frontend developer takes a task from TODO to INPROGRESS. works on it, once she thinks
    everything is completed and perhaps created a code review, sets this task as ready to be pulled

 3. another developer will see the task as ready to be pulled and move it to Code review, and so on.

 4. A QA will verify the task after Code Review is complete

 5. The PO will make sure to Validate the Business logic

 This Pallet is about the experience a person earns for working on something, by aggregating your experiences we're able to introduce an anonymous way of identifying yourself.
 
We tried to figure out the most fair way to earn experience, thinking also on how to resist to attacks by
 introducing a few constraints:

 1. we may not move our task to the next column: only another person may move forward this task to the next column
 2. Not anyone is capable of moving forward a task, but depending of the column, only an expert on that role may pull that task.

For example: A pull request may be approved only by an engineer, only tested by a QA. But what does it mean you're an engineer or a QA?
 
 What makes you expert on something? We're talking about roles, roles are not preset. Like a QA
 says " I am QA" or a root whitelist the user as QA. That's not what we want. Any person may work
 on any kind of task, once someone else approves your task, we're setting a milestone. Someone is
 saying "this person did a good job for this specific task in this specific phase". I'm setting a
 STAMP. This stamp, makes the person which work on this task earn some experience points.

 That will be our ranking, and considering we may work on different kind of tasks, and our stamps
 are given by different roles, we want to have a good granurality level keeping track of all
 those aspects in a multidimensional matrix:

  person given stamps by role | role1 | role 2 | role 3 | role 4 | role 5
 worked on kind of task:
 kind 1
 kind 2
 kind 3
 kind 4
 kind 5

 so for example, think on someone who is a bit QA, a bit developer.

 Alice worked on two tasks:
 1. as deveopler signup text color, changing from yellow to green.
 2. QA of a shopping cart functionality.

 Alice work on task 1, pull the task from TODO to INPROGRESS, once done set the task as ready for
 review. A frontend expert pulls the task and move it to "Code review", finally approves that
 task. Alice  work on task 2, pulling a task from CODE REVIEW DONE to QA, once done set QA to "QA
 DONE"


THE WORKFLOW: 

STEP 1: The process begins when the selection of a task, which is pulled from the `TO DO` list. A PRODUCT OWNER(PO) plays a pivotal role in this phase by marking the task as `READY to PULL`

STEP 2: Once the task is ready i.e., `READY to PULL` Alice takes ownership of it. Her initial responsibility is to create a "Prood of Positive Interaction" with the Product Owner(PO). Notably this proof materializes when Alice completes the task and designates it as `READY` for others to work on. This step is crucial because tasks initally initiated by the PO might lack quality or essential information. 

STEP 3: Alice (Developer) sets the task as `READY for REVIEW` and initiates a Code Review. At this point the Developer might pick up this task. However there is a possibility that the task still doesn't meet expectations or has quality issues

STEP 4: The Proof of Positive Interaction is generated for ALICE when the task receives an approved `CODE REVIEW`. This approval signifies that the task aligns with the required standards, reflecting the collaborative nature of the process. And the reviewer marks the task as `READY for QA`

STEP 5: A QA Professional pulls the task marked as `READY for QA`. They meticulously work on it and upon completion mark it as `QA DONE`. This action not only signifies the task's readyness for the next stage but also represents another Proof of Positive Interaction, this time between the QA specialist and the Developer.

STEP 6: The `QA DONE` task can now be picked up by the Product Owner(PO)/Developer and is validated to be put into `READY for RELEASE`. This again initiates a Proof of Positive interaction between the QA and the PO or the Dev. 

STEP 7: Finally the Task is deployed by a Release Engineer, Developer or someone from the DevOps team, this enhances the ranking of the PO. This steps illustrates the final outcome of the collaborative effort, with the PO benefiting from the successful completion and deployment of the task to Production and mark as `DONE` / `DEPLOYED to PRODUCTION`

 It's important to acknowledge that in our project, individual roles aren't solely a matter of personal choice but evolve through positive interactions. Initially, when the project begins, there may not be established experts. While one option is assigning roles at the outset, I favor a more inclusive approach. At the project's start, each team member commences with zero experience, yet they hold full task approval authority. We can develop a method that factors in the average role-specific experience, setting it as a minimum for task approvals. This way, the approval threshold grows alongside our team's expertise, fostering a supportive and collaborative environment. 

 Root(which could be for a specific project the PO), defines for a project two sets of values:
 1. kind of work(or in the board example, kind of tasks, for example a coding task,
 2. steps needed for a any work to be considered it DONE. In the board example, the columns of
    the board.
 3. The roles allowed to set a proof of positive interaction on some specific steps.
 For example in our board example, only a PO is allowed to move a task from NEW to TO DO
 Only a developer is allowed to allowed to pull and approve a code review. Or only a developer is
 allowed to deploy the task.

 As I said before, what defines the a person to be a developer, is based on her experience, that
 must be higher than the norm of all accounts in that specific role

# About identity

 Use case:
Proof of positive interaction as way to introduce trustless, anonymous way of providing access to resources or operations. 
 Use case:
 You join a company, they know who are you, and create an account for you, in order to give you the right access to some resources.
 Everything could be supersecure, following the most advanced guidelines about granurality on access to every single resource. Still
 there two very bad aspects in the use case I described before.
 1. They know who are you. What does it mean? you give an id card, then suddently you're the guy on that picture.
 We could do background checks, still we could at most, up to some extent, validate the companies you worked for. 

 2. once they decide up to some extend who are you, decide in a manual way, to create some accounts in order to provide access for you to things.

 We did great improvements in the security literature evolving from access to monoliths to instead restricting or granting access to every single resource that makes sense. It would be amazing if we could for example validate the capability to execute a task based on the passt story of that person, not on who that person is. In the end we just want the most capable person in something to to that specific thing. 
 * Non ideal identity about a person: Name, Software Engineer at CompanyX
 * Ideal: 
 AccountId(i.e. public key)
 Pull request X, 
 Approval Y, 
 Code review Z, 
 Given Talk about X, 
 Proof of attendance ZX

 Once we have a way to validate your actions as set of positive interactions between you and other actions. Considering we have the ranking of each of those actors, we may rely on those actors validation.

