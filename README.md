# Main

## Example directories
bak/packages.xml/

## Example filenames
```
packages.xml.20161213-093146_r575055
packages.xml.20181102-144204
packages.xml.20181105-103813
```
The format is of the file is:
```
name.extension.YYYYMMDD-HHMMSS[_rNNNNNNN]
```

## special files
packages.xml_swinstall_stack

```xml
<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
</stack_history>
```

## Problem Statement
- given a datetime find the file that is less than or equal to a supplied datetime from a set of candidates which are upper bounded by the current item in the swinstall_stack