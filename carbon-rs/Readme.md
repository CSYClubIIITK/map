
# Malware Analysis Project-Carbon-rs

WARNING: THIS REPOSITORY CONTAINS MALWARE SAMPLES AND IS TO BE USED ONLY FOR EDUCATIONAL RECREATIONAL PURPOSES.
ALMOST ALL MALWARE SAMPLES HERE ARE WELL DOCUMENTED AND ANY ANTI-VIRUS WILL DETECT THEM, SO DO NOT USE THIS REPOSITORY IN ANY MALICIOUS MANNER



![Logo](https://i.imgur.com/yo7LuuQ.png)


## Appendix

This is a Project undertaken by CSY-Club IIITK to help students get a more hands-on approach to malware and how modern day threats work.


## Documentation

carbon-rs has been written by us at CSY-Club IIITK as a showcase virus, to explain the workings of WIN32 API and how exactly a common type of Malware Functions.
## Usage/Examples
DISABLE WINDOWS DEFENDER OR ANY OTHER ANTIVIRUS IN YOUR COMPUTER BEFORE YOU RUN THIS
You can trust me/US :))))

The app takes one argument, time.
This attribute specifies the amount of time the APP runs.

an example run command to run it for 60 seconds:
```javascript
./Maldev -t 60
```

The output is saved in two files:
    Keys.txt
    and
    Windows.txt



## WIN32 API

Carbon-rs uses WIN32 API, It's basically a library provided by Microsoft to interact with the OS using code and is used by most popular applications today.

This will be our Holy book for Initial stages of this project.

As an Introduction to the API, we'll Look at 2 functions that are used to replicate the malware we used
## API Reference

The entire API documentation can be found at https://learn.microsoft.com/en-us/windows/win32/api/

For this Project the following API calls have been used
#### GetForegroundWindow

```http
HWND GetForegroundWindow();
```
This is a function, if u call it, it will directly give the window that is open right now

In the code I am repetedly calling this function every second and returning info from it.


#### GetKeyState

```c++
LRESULT CALLBACK LowLevelKeyboardProc(int nCode, WPARAM wParam, LPARAM lParam) {
    // Access keyboard information from lParam
    KBDLLHOOKSTRUCT* pKeyStruct = (KBDLLHOOKSTRUCT*)lParam;

    // Process the keyboard event based on wParam and pKeyStruct
    // For example, check for specific key presses or combinations

    return CallNextHookEx(NULL, nCode, wParam, lParam);
}

```
This is also a function that return the keys and has to be called in a loop.



## Lessons Learned

An obvious thing to notice is that this malware gets detected by windows defender and almost any other anti-virus software.

### Reasons
Becuase the WIN32 API is so well known, almost all Antivirus will be able to detect any malware we write using WIN32 API

### So why Learn this?
WIN32 API is an easy way to replicate a lot of things advanced malware do, although they might use more sophisticated ways to acheive the same thing.

Malware Development is less about coding and more about how creative we can get in writing malware to make it undetectable. Learning WIN32 API based malware is a way for us to experiment with how creative we can get with out development, without getting too caught up in Technical Implementations.
## Feedback

If you have any feedback or security related concerns, please reach out to us at csyclub@iiitkottayam.ac.in


## Authors

- [@kol-mikaelson](https://www.github.com/kol-mikaelson)


## Acknowledgements
- WIN32 API Documentation
- Crow.rip
- Blackhat Rust

