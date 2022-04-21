# rust_waitable_timer
Quick and dirty test of CreateWaitableTimerExW Windows API in Rust.

Implementation is in winsleep.rs and a simple test in main.rs.

Execute with ```cargo run```.

## Example output of cargo run
```
min 1, max 1000000
=== thread::sleep ===
expected: 10000 us
  actual: 18715 us
  actual: 15782 us
  actual: 15402 us
  actual: 15664 us
  actual: 15637 us
  actual: 15593 us
  actual: 15661 us
  actual: 15651 us
  actual: 15645 us
expected: 3000 us
  actual: 15462 us
  actual: 14625 us
  actual: 14608 us
  actual: 15911 us
  actual: 14839 us
  actual: 14679 us
  actual: 15171 us
  actual: 15407 us
  actual: 15447 us
expected: 2000 us
  actual: 14917 us
  actual: 15307 us
  actual: 15147 us
  actual: 14689 us
  actual: 15478 us
  actual: 14789 us
  actual: 15309 us
  actual: 16144 us
  actual: 15520 us
expected: 1000 us
  actual: 15405 us
  actual: 14316 us
  actual: 16011 us
  actual: 14736 us
  actual: 15337 us
  actual: 14845 us
  actual: 15351 us
  actual: 15602 us
  actual: 14926 us
expected: 500 us
  actual: 15502 us
  actual: 15138 us
  actual: 15170 us
  actual: 15265 us
  actual: 15181 us
  actual: 15162 us
  actual: 15621 us
  actual: 14807 us
  actual: 15907 us
expected: 250 us
  actual: 14564 us
  actual: 15121 us
  actual: 14803 us
  actual: 15230 us
  actual: 14747 us
  actual: 15161 us
  actual: 15215 us
  actual: 15777 us
  actual: 15172 us
=== windows_sleep ===
expected: 10000 us
  actual: 10571 us
  actual: 10074 us
  actual: 10875 us
  actual: 10074 us
  actual: 10900 us
  actual: 10617 us
  actual: 10679 us
  actual: 11003 us
  actual: 10623 us
expected: 3000 us
  actual: 3256 us
  actual: 3484 us
  actual: 3862 us
  actual: 3834 us
  actual: 3695 us
  actual: 3733 us
  actual: 4180 us
  actual: 3945 us
  actual: 3636 us
expected: 2000 us
  actual: 2636 us
  actual: 2645 us
  actual: 2871 us
  actual: 2673 us
  actual: 2645 us
  actual: 2773 us
  actual: 2814 us
  actual: 2893 us
  actual: 2370 us
expected: 1000 us
  actual: 1928 us
  actual: 1518 us
  actual: 1766 us
  actual: 1848 us
  actual: 1846 us
  actual: 1954 us
  actual: 1798 us
  actual: 1803 us
  actual: 1802 us
expected: 500 us
  actual: 528 us
  actual: 530 us
  actual: 532 us
  actual: 529 us
  actual: 530 us
  actual: 527 us
  actual: 520 us
  actual: 520 us
  actual: 521 us
expected: 250 us
  actual: 523 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
  actual: 520 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
=== thread::sleep after beginTimePeriod ===
expected: 10000 us
  actual: 11018 us
  actual: 10967 us
  actual: 10536 us
  actual: 10877 us
  actual: 10234 us
  actual: 10514 us
  actual: 10733 us
  actual: 10112 us
  actual: 10228 us
expected: 3000 us
  actual: 3483 us
  actual: 3598 us
  actual: 3779 us
  actual: 3026 us
  actual: 3671 us
  actual: 3761 us
  actual: 3764 us
  actual: 3797 us
  actual: 3520 us
expected: 2000 us
  actual: 2500 us
  actual: 2638 us
  actual: 2810 us
  actual: 2840 us
  actual: 2673 us
  actual: 2706 us
  actual: 2330 us
  actual: 2798 us
  actual: 2842 us
expected: 1000 us
  actual: 1615 us
  actual: 1794 us
  actual: 1688 us
  actual: 1790 us
  actual: 1794 us
  actual: 1765 us
  actual: 1819 us
  actual: 1845 us
  actual: 1768 us
expected: 500 us
  actual: 1545 us
  actual: 1685 us
  actual: 1792 us
  actual: 1793 us
  actual: 1793 us
  actual: 1773 us
  actual: 1735 us
  actual: 1847 us
  actual: 1667 us
expected: 250 us
  actual: 1534 us
  actual: 1793 us
  actual: 1796 us
  actual: 1795 us
  actual: 1763 us
  actual: 1781 us
  actual: 1852 us
  actual: 1788 us
  actual: 1671 us
=== windows_sleep after beginTimePeriod ===
expected: 10000 us
  actual: 10924 us
  actual: 10515 us
  actual: 10304 us
  actual: 10659 us
  actual: 10501 us
  actual: 10723 us
  actual: 10287 us
  actual: 10513 us
  actual: 10079 us
expected: 3000 us
  actual: 3066 us
  actual: 3954 us
  actual: 3482 us
  actual: 3789 us
  actual: 3558 us
  actual: 3188 us
  actual: 3697 us
  actual: 3781 us
  actual: 3182 us
expected: 2000 us
  actual: 2598 us
  actual: 2531 us
  actual: 2850 us
  actual: 2789 us
  actual: 2795 us
  actual: 2989 us
  actual: 2613 us
  actual: 2663 us
  actual: 2859 us
expected: 1000 us
  actual: 1563 us
  actual: 1990 us
  actual: 1862 us
  actual: 1912 us
  actual: 1831 us
  actual: 1843 us
  actual: 1710 us
  actual: 1812 us
  actual: 1744 us
expected: 500 us
  actual: 532 us
  actual: 524 us
  actual: 521 us
  actual: 540 us
  actual: 531 us
  actual: 522 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
expected: 250 us
  actual: 522 us
  actual: 522 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
  actual: 521 us
  actual: 528 us
  actual: 531 us
```
