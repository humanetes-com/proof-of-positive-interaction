License: MIT-0

 # POPI Pallet

 Proof of positive interaction as way to introduce trustless, anonymous way of providing access to resources or operations. 
 Use case:
 You join a company, they know who are you, and create an account for you, in order to give you the right access to some resources.
 Everything could be supersecure, following the most advanced guidelines about granurality on access to every single resource. Still
 there two very bad aspects in the use case I described before.
 1. They know who are you. What does it mean? you give an id card, then suddently you're the guy on that picture.
 We could do background checks, still we could at most, up to some extent, validate the companies you worked for. 
 
 2. once they decide up to some extend who are you, decide in a manual way, to create some accounts in order to provde access for you to things.
 
 Just like we did gret improvements in the security literature by giving access not to monoliths, instead to every single resource, it would be amazing if we could for example validate every single task this person did. Wouldn't this be more expressfull of who you are?
 * Non ideal identity about a person: Name, Software Engineer at CompanyX
 * Ideal: Name, 
 Pull request X, 
 Approval Y, 
 Code review Z, 
 Given Talk about X, 
 Proof of attendance ZX
 
 Once we have a way to validate your actions as set of positive interactions between you and other actions. Considering we have the ranking of each of those actors, we may rely on those actors validation.
 
*Where are we different from Keybase

Actually keybase 
*Differences with Gitcoin

*Differences with Sourcecreed


 A collectivity that contributes toward a common goal.
 There are different kind of products that may be built, each will go through
 a set of lavoration steps. A lavoration step is registered onchain through an interaction.
 The interaction is another user that verifies positively that your work contributed to move
 a thing from the lavoration step n to the n+1.

Lets make a concrete example in order to help you visualize the concepts:
An 
ecommerce software company, the website is built following a
 kanban or scrum agile process. Think at the board typical columns:

 NEW	 | TODO | IN PROGRESS | CODE REVIEW | QA | READY TO DEPLOY | DEPLOYED | QA PRODUCTION | DONE
        Task1
 Task2
                              Task3
 ....

 We have tasks, task can be of 3 different kinds:
 1. UI = design the UI of a functionality
 2. FRONTEND DEVELOP = i.e. coding in React.js
 3. SERVER DEVELOP = i.e. implementing an API service consumed by the frontend
 4. BI = someone who analyses the user journey
 5. QA = Someone who validates something built
 6. PO = creator of tasks.
 7. PM = project manager

 Every task must follow all the steps from NEW to DONE. The progress can never be done by the
 same person who worked on this specific task in this specific column.

 Lets make an example:
 Task: change a color of a text "Click here to signin" from RED To Yellow.

 This task is created by UI. The task goes inmediately to the NEW column.
 PO moves this to TODO.

 A developer PULLS this task, means: moves this task from TODO into IN PROGRESS>
 Once she is done, set the task into "COMPLETED the current step". So small detail:
 yes, someone else must move the task, but the person in this step must say "MY WORK IS READY TO
 BE PULLED" So the board is simplified but actually it has the double of steps:
 NEW| READY TO BE PULLED TO TODO | TODO| READY FOR INPROGRESS | INPROGRESS | READY FOR REVIEW |
 REVIEW |

 three cases connected to the previous example:
 1. UI creates the task, this task must be well described, contain the color code corresponding
    to
 Yellow. When she thinks the task is ready to be pulled by A PO, set the task as "READY TO BE
 PULLED"

 2. When a frontend developer takes a task from TODO to INPROGRESS. works on it, once she thinks
    everything is
 completed and perhaps created a code review, sets this task as ready to be pulled

 3. another developer will see the task as ready to be pulled and move it to Code review,
 and so on.

 This Pallet is about the experience a person earns for working on something. We tried to figure
 out the must fair way to earn experience, thinking also on how to resist to attacks by
 introducing a few constraints:

 I already mentioned the first constraint, the most important: we work on a specific phase of a
 task, set this as ready, but only another person may pull this task. Not anyone is capable of
 pulling a task, but depending of the column, only an expert on that role may pull that task.
 What makes you expert on something? We're talking about roles, roles are not preset. Like a QA
 says " I am QA" or a root whitelist the user as QA. Tha'ts not what we want. Any person may work
 on any kind of task, once someone else approves your task, we're setting a milestone. Someone is
 saying "this person did a good job for this specific task in this specific phase". I'm setting a
 STAMP. This stamp, makes the person which work on this task earn some experience points.

 That will be our ranking, and considering we may work on different kind of tasks, and our stamps
 are given by different roles, we want to have a good granurality level keeping track of all
 those aspects in a multidimensional matrix:

  person given stamps by role | role1 | role 2 | role 3 | role 4
 worked on kind of task:
 kind 1
 kind 2
 kind 3
 kind 4

 so for example, think on someone who is a bit QA, a bit developer.

 Alice worked on two tasks:
 1. as deveopler signup text color, changing from yellow to green.
 2. QA of a shopping cart functionality.

 Alice work on task 1, pull the task from TODO to INPROGRESS, once done set the task as ready for
 review. A frontend expert pulls the task and move it to "Code review", finally approves that
 task. Alice  work on task 2, pulling a task from CODE REVIEW DONE to QA, once done set QA to "QA
 DONE"

 Many roles are involved in this process:
 a. Task 1 pulled from TODO, was set as ready by a PO. Once Alice pulls it, is creating a proof
 of positive interaction to the PO. Actually thinking on it, the proof is given just once Alice
 finalize her task, and set it as READY to be pulled. cause the task creatd by the PO could have
 low quality, not contain all the info needed and Alice never being able to finalize it. The next
 role will make this point clearer: b. she sets the task as READY to be reviewed, cretes a code
 review.A developer pulls it by movindg the task to CODE REVIEW, and the task could be low
 quality, or not meet the espectations of the task. Only when the task will have an approved code
 review, so the actual code review approval, will register a proof of positive interaction to
 ALICE. c. once the code review is set as READY to be pulled, a QA will pull this task, work on
 it, once the QA is ready, will set as QA done. Again, setting a positive proof of interaction
 with the developer. d. a QA ready task, could be pulled perhaps by a PO(Product owner) or even
 by a developer and moved to "READY TO BE RELEASED" e. when the task is deployed by a release
 engineer or a developer, or a devops, this will increase the ranking of the PO.

 I hope now it is clear when I say that a person don't decides her role. Her role is an
 experience depending on positive proof of interactiosn with other experts. Of course initially
 in the project there will be no experts. Perhaps a ROOT could define initial roles, but I like
 more the idea of initially every person has 0 as experience, and is allowed to approve
 everything. We could define a formula that takes the norm of the experience on each role and
 sets this as a minimum for approving a task. In this way the experience needed to approve will
 be variable and grow with the overall experience of the community.

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
