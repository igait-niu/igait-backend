# AI-ASD
todo! - description (gather full criteria first)

## Control Flow
![image](https://github.com/hiibolt/ai-asd/assets/91273156/e6fb254d-0fb4-454d-80c6-b76bb7850202)

Due to the nature of our team likely using an unconventional inference method, I chose to seperate the backend from interference code entirely. In the event this prevents proper model caching, I will rework the ``run_interference`` section.

The state machine instead uses a queue, holding videos in ``./data/queue`` until they are next in line. When a job is frontal, it's passed as an argument into the command interface designated in ``run_interference``.

## Access
There are **3** methods of access. (HTML, iOS app, Android app)
- ### HTML
  Currently, the HTML app is proof-of-concept. However, it is fully functional. 
  
  Upon submitting a file, it is uploaded to the server, and the returned ID is stored locally. The page refreshes each stored job ID every 500ms.
  
  ![image](https://github.com/hiibolt/ai-asd/assets/91273156/a555dd0e-0140-4511-85a4-26e3deefbfb0)

- ### iOS
  todo! - complete iOS boilerplate

- ### Android
  todo! - complete Android boilerplate
