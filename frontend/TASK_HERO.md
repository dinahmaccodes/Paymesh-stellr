Okay so we are working on a frontend task as well. 



Implement a pixel-perfect waitlist screen that matches the Figma design specifications. This component will be the entry point for users to join the Paymesh waitlist.



Core Requirements

Pixel-Perfect Implementation

Match the Figma design exactly

Accurate spacing, typography, and colors

Proper alignment and layout

Correct use of design tokens (if applicable)

Responsive Design

Mobile-first approach

Smooth transitions between breakpoints

Maintain design integrity across all screen sizes

Test on common viewport sizes (mobile, tablet, desktop)

Functionality

Implement waitlist form with proper validation

Handle user input correctly

Show appropriate success/error states

Ensure smooth user interactions

Code Quality

Keep component under 150 lines of code (as per CONTRIBUTING.md)

Follow professional frontend standards

Write clean, optimized code

Use semantic HTML

Implement proper accessibility features



Implement changes

Study the Figma design thoroughly

Note all spacing values

Extract exact colors and fonts

Identify all states (default, hover, focus, error, success)

Check for animations or transitions

Create the component

Build component: components/waitlist-screen.tsx (or appropriate path)

Implement form validation logic

Add proper input handling

Include loading and success states

Ensure error handling

Style the component

Match Figma specifications exactly

Implement responsive breakpoints

Add hover and focus states

Test on multiple devices/screen sizes

Test thoroughly

Verify form validation works correctly

Test responsiveness at all breakpoints

Check accessibility (keyboard navigation, screen readers)

Validate visual accuracy against Figma

Test and commit

Compare implementation with Figma side-by-side

Test form submission flow

Verify responsive behavior

Include screenshots or gifs in the PR showing:

Desktop view

Tablet view

Mobile view

Different states (default, filled, error, success)

Guidelines

Read the CONTRIBUTING.md guide before applying - Understand our standards and workflow

Assignment required before starting

PR description must include: Closes #[issue_id]

Specify your ETA when applying (Maximum 48 hours)

After 24 hours, a draft PR or finished work is expected

Component must not exceed 150 lines of code

Follow the established frontend structure

Maintain code quality and optimization





Here is some code details from the figma design 



```

/* Hero */

/* Auto layout */

display: flex;

flex-direction: column;

justify-content: space-between;

align-items: center;

padding: 0px;

isolation: isolate;

position: relative;

width: 1440px;

height: 815px;

background: linear-gradient(180deg, #030407 0%, #111827 100%);

/* Stars */

position: absolute;

width: 1440px;

height: 815px;

left: 0px;

top: 0px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

z-index: 0;

/* Star 1 */

position: absolute;

width: 4px;

height: 4px;

left: 252px;

top: 0px;

background: #4B4B4B;

/* Star 27 */

position: absolute;

width: 4px;

height: 4px;

left: 246.2px;

top: 140px;

background: #4B4B4B;

transform: rotate(76.75deg);

/* Star 79 */

position: absolute;

width: 4px;

height: 4px;

left: 0px;

top: 598px;

background: #4B4B4B;

/* Star 2 */

position: absolute;

width: 4px;

height: 4px;

left: 287px;

top: 70px;

background: #4B4B4B;

/* Star 28 */

position: absolute;

width: 4px;

height: 4px;

left: 108.91px;

top: 228.84px;

background: #4B4B4B;

transform: rotate(76.75deg);

/* Star 54 */

position: absolute;

width: 4px;

height: 4px;

left: 642.2px;

top: 453px;

background: #4B4B4B;

/* Star 80 */

position: absolute;

width: 4px;

height: 4px;

left: 35px;

top: 668px;

background: #4B4B4B;

/* Star 3 */

position: absolute;

width: 4px;

height: 4px;

left: 360px;

top: 50px;

background: #4B4B4B;

/* Star 29 */

position: absolute;

width: 4px;

height: 4px;

left: 145.11px;

top: 295.31px;

background: #4B4B4B;

transform: rotate(76.75deg);

/* Star 55 */

position: absolute;

width: 4px;

height: 4px;

left: 919.06px;

top: 645.76px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 81 */

position: absolute;

width: 4px;

height: 4px;

left: 108px;

top: 648px;

background: #4B4B4B;

/* Star 6 */

position: absolute;

width: 4px;

height: 4px;

left: 509px;

top: 54px;

background: #4B4B4B;

/* Star 30 */

position: absolute;

width: 4px;

height: 4px;

left: 313px;

top: 272px;

background: #4B4B4B;

/* Star 56 */

position: absolute;

width: 4px;

height: 4px;

left: 836.86px;

top: 939.95px;

background: #4B4B4B;

transform: rotate(27.89deg);

/* Star 82 */

position: absolute;

width: 4px;

height: 4px;

left: 214px;

top: 590px;

background: #4B4B4B;

/* Star 9 */

position: absolute;

width: 4px;

height: 4px;

left: 658px;

top: 58px;

background: #4B4B4B;

/* Star 31 */

position: absolute;

width: 4px;

height: 4px;

left: 502px;

top: 276px;

background: #4B4B4B;

/* Star 57 */

position: absolute;

width: 4px;

height: 4px;

left: 737.33px;

top: 689.73px;

background: #4B4B4B;

transform: rotate(27.89deg);

/* Star 12 */

position: absolute;

width: 4px;

height: 4px;

left: 649.68px;

top: 235.13px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 84 */

position: absolute;

width: 4px;

height: 4px;

left: 787px;

top: 561.84px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 15 */

position: absolute;

width: 4px;

height: 4px;

left: 781.95px;

top: 109.12px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 33 */

position: absolute;

width: 4px;

height: 4px;

left: 524.63px;

top: 639.47px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 59 */

position: absolute;

width: 4px;

height: 4px;

left: 1659.5px;

top: 590.69px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 85 */

position: absolute;

width: 4px;

height: 4px;

left: 664.33px;

top: 687.47px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 18 */

position: absolute;

width: 4px;

height: 4px;

left: 844.22px;

top: 51.12px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 60 */

position: absolute;

width: 4px;

height: 4px;

left: 1050.29px;

top: 554px;

background: #4B4B4B;

/* Star 86 */

position: absolute;

width: 4px;

height: 4px;

left: 623.54px;

top: 761.23px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 21 */

position: absolute;

width: 4px;

height: 4px;

left: 1254px;

top: 74px;

background: #4B4B4B;

/* Star 35 */

position: absolute;

width: 4px;

height: 4px;

left: 1022.38px;

top: 272.33px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 61 */

position: absolute;

width: 4px;

height: 4px;

left: 1231px;

top: 769.7px;

background: #4B4B4B;

/* Star 87 */

position: absolute;

width: 4px;

height: 4px;

left: 1461.86px;

top: 393.07px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 24 */

position: absolute;

width: 4px;

height: 4px;

left: 1403px;

top: 78px;

background: #4B4B4B;

/* Star 36 */

position: absolute;

width: 4px;

height: 4px;

left: 1250px;

top: 358px;

background: #4B4B4B;

/* Star 62 */

position: absolute;

width: 4px;

height: 4px;

left: 1380px;

top: 773.7px;

background: #4B4B4B;

/* Star 88 */

position: absolute;

width: 4px;

height: 4px;

left: 1256.07px;

top: 660px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 4 */

position: absolute;

width: 4px;

height: 4px;

left: 395px;

top: 120px;

background: #4B4B4B;

/* Star 37 */

position: absolute;

width: 4px;

height: 4px;

left: 85px;

top: 345.42px;

background: #4B4B4B;

transform: rotate(76.75deg);

/* Star 89 */

position: absolute;

width: 4px;

height: 4px;

left: 143px;

top: 718px;

background: #4B4B4B;

/* Star 7 */

position: absolute;

width: 4px;

height: 4px;

left: 584px;

top: 124px;

background: #4B4B4B;

/* Star 38 */

position: absolute;

width: 4px;

height: 4px;

left: 348px;

top: 342px;

background: #4B4B4B;

/* Star 64 */

position: absolute;

width: 4px;

height: 4px;

left: 796.98px;

top: 804.94px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 90 */

position: absolute;

width: 4px;

height: 4px;

left: 249px;

top: 660px;

background: #4B4B4B;

/* Star 10 */

position: absolute;

width: 4px;

height: 4px;

left: 748px;

top: 94px;

background: #4B4B4B;

/* Star 39 */

position: absolute;

width: 4px;

height: 4px;

left: 418px;

top: 632px;

background: #4B4B4B;

/* Star 65 */

position: absolute;

width: 4px;

height: 4px;

left: 850.2px;

top: 944.17px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 13 */

position: absolute;

width: 4px;

height: 4px;

left: 681.3px;

top: 266.69px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 66 */

position: absolute;

width: 4px;

height: 4px;

left: 1554.97px;

top: 510.54px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 92 */

position: absolute;

width: 4px;

height: 4px;

left: 1002px;

top: 603.08px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 16 */

position: absolute;

width: 4px;

height: 4px;

left: 853.57px;

top: 140.68px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 41 */

position: absolute;

width: 4px;

height: 4px;

left: 832.46px;

top: 374.9px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 67 */

position: absolute;

width: 4px;

height: 4px;

left: 1499.18px;

top: 677.47px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 93 */

position: absolute;

width: 4px;

height: 4px;

left: 613px;

top: 746.55px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 19 */

position: absolute;

width: 4px;

height: 4px;

left: 1140px;

top: 140px;

background: #4B4B4B;

/* Star 68 */

position: absolute;

width: 4px;

height: 4px;

left: 927px;

top: 362px;

background: #4B4B4B;

/* Star 94 */

position: absolute;

width: 4px;

height: 4px;

left: 572.21px;

top: 820.31px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 22 */

position: absolute;

width: 4px;

height: 4px;

left: 1289px;

top: 144px;

background: #4B4B4B;

/* Star 69 */

position: absolute;

width: 4px;

height: 4px;

left: 1266px;

top: 839.7px;

background: #4B4B4B;

/* Star 95 */

position: absolute;

width: 4px;

height: 4px;

left: 1410.54px;

top: 452.15px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 25 */

position: absolute;

width: 4px;

height: 4px;

left: 1438px;

top: 148px;

background: #4B4B4B;

/* Star 44 */

position: absolute;

width: 4px;

height: 4px;

left: 1285px;

top: 428px;

background: #4B4B4B;

/* Star 70 */

position: absolute;

width: 4px;

height: 4px;

left: 1415px;

top: 843.7px;

background: #4B4B4B;

/* Star 96 */

position: absolute;

width: 4px;

height: 4px;

left: 1204.75px;

top: 719.08px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 5 */

position: absolute;

width: 4px;

height: 4px;

left: 470px;

top: 190px;

background: #4B4B4B;

/* Star 45 */

position: absolute;

width: 4px;

height: 4px;

left: 297px;

top: 554px;

background: #4B4B4B;

/* Star 97 */

position: absolute;

width: 4px;

height: 4px;

left: 178px;

top: 788px;

background: #4B4B4B;

/* Star 8 */

position: absolute;

width: 4px;

height: 4px;

left: 619px;

top: 194px;

background: #4B4B4B;

/* Star 46 */

position: absolute;

width: 4px;

height: 4px;

left: 383px;

top: 412px;

background: #4B4B4B;

/* Star 72 */

position: absolute;

width: 4px;

height: 4px;

left: 856.1px;

top: 764.03px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 73 */

position: absolute;

width: 4px;

height: 4px;

left: 798.87px;

top: 1003.25px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 99 */

position: absolute;

width: 4px;

height: 4px;

left: 680.37px;

top: 514.64px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 48 */

position: absolute;

width: 4px;

height: 4px;

left: 501px;

top: 520.92px;

background: #4B4B4B;

transform: rotate(27.89deg);

/* Star 74 */

position: absolute;

width: 4px;

height: 4px;

left: 1503.64px;

top: 569.62px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 100 */

position: absolute;

width: 4px;

height: 4px;

left: 398.02px;

top: 766.4px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 17 */

position: absolute;

width: 4px;

height: 4px;

left: 870.19px;

top: 206.24px;

background: #4B4B4B;

transform: rotate(-39.65deg);

/* Star 75 */

position: absolute;

width: 4px;

height: 4px;

left: 482.8px;

top: 658.66px;

background: #4B4B4B;

/* Star 101 */

position: absolute;

width: 4px;

height: 4px;

left: 451.23px;

top: 905.63px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 20 */

position: absolute;

width: 4px;

height: 4px;

left: 1175px;

top: 210px;

background: #4B4B4B;

/* Star 76 */

position: absolute;

width: 4px;

height: 4px;

left: 962px;

top: 432px;

background: #4B4B4B;

/* Star 102 */

position: absolute;

width: 4px;

height: 4px;

left: 1306px;

top: 372px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 23 */

position: absolute;

width: 4px;

height: 4px;

left: 1324px;

top: 214px;

background: #4B4B4B;

/* Star 51 */

position: absolute;

width: 4px;

height: 4px;

left: 1171px;

top: 494px;

background: #4B4B4B;

/* Star 77 */

position: absolute;

width: 4px;

height: 4px;

left: 1301px;

top: 909.7px;

background: #4B4B4B;

/* Star 103 */

position: absolute;

width: 4px;

height: 4px;

left: 1359.21px;

top: 511.23px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Star 26 */

position: absolute;

width: 4px;

height: 4px;

left: 1473px;

top: 218px;

background: #4B4B4B;

/* Star 52 */

position: absolute;

width: 4px;

height: 4px;

left: 1320px;

top: 498px;

background: #4B4B4B;

/* Star 78 */

position: absolute;

width: 4px;

height: 4px;

left: 1450px;

top: 913.7px;

background: #4B4B4B;

/* Star 104 */

position: absolute;

width: 4px;

height: 4px;

left: 1121.72px;

top: 566.46px;

background: #4B4B4B;

transform: rotate(67.55deg);

/* Nav */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: space-between;

align-items: center;

padding: 20px 100px;

gap: 84px;

margin: 0 auto;

width: 1440px;

height: 63px;

background: rgba(5, 7, 11, 0.1);

backdrop-filter: blur(5px);

/* Note: backdrop-filter has minimal browser support */

/* Inside auto layout */

flex: none;

order: 1;

align-self: stretch;

flex-grow: 0;

z-index: 1;

/* Logo */

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 0px;

gap: 4px;

margin: 0 auto;

width: 120px;

height: 23px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Group 1 */

width: 35px;

height: 21px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Line 1 */

position: absolute;

width: 35px;

height: 6.06px;

left: 0px;

top: 1px;

border: 2.12766px solid #FFFFFF;

/* Line 2 */

position: absolute;

width: 35px;

height: 6.06px;

left: 0px;

top: 8.47px;

border: 2.12766px solid #FFFFFF;

/* Line 3 */

position: absolute;

width: 35px;

height: 6.06px;

left: 0px;

top: 15.94px;

border: 2.12766px solid #FFFFFF;

/* Paymesh */

width: 81px;

height: 23px;

font-family: 'DM Sans';

font-style: normal;

font-weight: 700;

font-size: 18px;

line-height: 23px;

color: #FFFFFF;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* Hero content */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: column;

justify-content: space-between;

align-items: center;

padding: 60px 100px;

gap: 80px;

margin: 0 auto;

width: 1440px;

height: 689px;

/* Inside auto layout */

flex: none;

order: 2;

align-self: stretch;

flex-grow: 1;

z-index: 2;

/* Hero text */

/* Auto layout */

display: flex;

flex-direction: column;

justify-content: center;

align-items: center;

padding: 0px;

gap: 20px;

margin: 0 auto;

width: 1023px;

height: 99px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Be the first to hear from us! */

width: 416px;

height: 39px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 500;

font-size: 34px;

line-height: 39px;

/* identical to box height */

text-align: center;

background: linear-gradient(90deg, #DFDFE0 0%, #282B31 100%);

-webkit-background-clip: text;

-webkit-text-fill-color: transparent;

background-clip: text;

text-fill-color: transparent;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Socials */

/* Auto layout */

display: flex;

flex-direction: row;

align-items: center;

padding: 0px;

gap: 12px;

width: 92px;

height: 40px;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* Twitter */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

align-items: center;

padding: 12px;

gap: 10px;

width: 40px;

height: 40px;

border-radius: 4px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* XLogo */

width: 18px;

height: 18px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Vector */

position: absolute;

left: 15.6%;

right: 15.63%;

top: 12.5%;

bottom: 12.45%;

background: #FFFFFF;

/* Github */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

align-items: center;

padding: 12px;

gap: 10px;

width: 40px;

height: 40px;

border-radius: 4px;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* GithubLogo */

width: 18px;

height: 18px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Vector */

position: absolute;

left: 0%;

right: 15.62%;

top: 9.38%;

bottom: 6.25%;

background: #FFFFFF;

/* Logo */

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 0px;

gap: 10.8px;

margin: 0 auto;

width: 322.34px;

height: 63px;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* Group 1 */

width: 94.54px;

height: 56.72px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Line 1 */

position: absolute;

width: 94.54px;

height: 16.37px;

left: 0px;

top: 3.14px;

border: 5.74693px solid #FFFFFF;

/* Line 2 */

position: absolute;

width: 94.54px;

height: 16.37px;

left: 0px;

top: 23.32px;

border: 5.74693px solid #FFFFFF;

/* Line 3 */

position: absolute;

width: 94.54px;

height: 16.37px;

left: 0px;

top: 43.49px;

border: 5.74693px solid #FFFFFF;

/* Paymesh */

width: 217px;

height: 63px;

font-family: 'DM Sans';

font-style: normal;

font-weight: 700;

font-size: 48.6191px;

line-height: 63px;

color: #FFFFFF;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* overview */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: space-between;

align-items: center;

padding: 12px 12px 12px 24px;

gap: 10px;

margin: 0 auto;

width: 500px;

height: 60px;

background: rgba(255, 255, 255, 0.05);

backdrop-filter: blur(5px);

/* Note: backdrop-filter has minimal browser support */

border-radius: 4px;

/* Inside auto layout */

flex: none;

order: 2;

flex-grow: 0;

/* Enter email address */

margin: 0 auto;

width: 141px;

height: 18px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 400;

font-size: 16px;

line-height: 18px;

color: #8398AD;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Primary Button */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 12px 24px;

gap: 10px;

margin: 0 auto;

width: 110px;

height: 40px;

background: rgba(250, 250, 250, 0.1);

backdrop-filter: blur(5px);

/* Note: backdrop-filter has minimal browser support */

border-radius: 4px;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* Launch App */

width: 62px;

height: 16px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 400;

font-size: 14px;

line-height: 16px;

/* identical to box height */

color: #E2E2E2;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Nav */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 20px 100px;

gap: 24px;

margin: 0 auto;

width: 1440px;

height: 63px;

background: #0A1223;

/* Inside auto layout */

flex: none;

order: 3;

align-self: stretch;

flex-grow: 0;

z-index: 3;

/* Nav text */

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 0px 24px;

gap: 10px;

width: 90px;

height: 18px;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Home */

width: 42px;

height: 18px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 400;

font-size: 16px;

line-height: 18px;

text-decoration-line: underline;

color: #E2E2E2;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Secondary Button */

box-sizing: border-box;

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 12px 24px;

gap: 10px;

width: 123px;

height: 40px;

background: rgba(250, 250, 250, 0.1);

backdrop-filter: blur(5px);

/* Note: backdrop-filter has minimal browser support */

border-radius: 4px;

/* Inside auto layout */

flex: none;

order: 1;

flex-grow: 0;

/* Launch App */

width: 75px;

height: 16px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 400;

font-size: 14px;

line-height: 16px;

/* identical to box height */

color: #FFFFFF;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;

/* Nav text */

/* Auto layout */

display: flex;

flex-direction: row;

justify-content: center;

align-items: center;

padding: 0px 24px;

gap: 10px;

width: 102px;

height: 18px;

/* Inside auto layout */

flex: none;

order: 2;

flex-grow: 0;

/* Waitlist */

width: 54px;

height: 18px;

font-family: 'GT Walsheim Trial';

font-style: normal;

font-weight: 400;

font-size: 16px;

line-height: 18px;

color: #8398AD;

/* Inside auto layout */

flex: none;

order: 0;

flex-grow: 0;



```



There is an X logo in   #xlogo.svg



Work on this task as a senior frontend developer who usese updated methods but also does not make things too lengthy when not needed. 



The wautlist probably fits into a certain feature on the webpage as the page.tsx has several components there for the sections of it's page. 



Here is an attached image 



/home/dinahmaccodes/Downloads/Hero.png









