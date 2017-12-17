# Alban
Alban is a simple tool to distribute dentistry students to their courses. It is a remittance work and serves a very specialized purpose. It is probably only applicable to the constrains of the University Rostock, Germany.  
Alban reads in a json file like it is described in the following chapter and wil output its results to stdout and "Alban says.txt"

# Input
The input of alban is a json file with the constraints of the distribution. Possible values are:
- studentenAnzahl: int - the total count of students that should be distributed
- wochen: object - with the following members
    - kwAnfang: int - the number of the first week
    - kwEnde: int - the number of the last week
    - the last week is required to be greater than the first (making a change in the year a bit inconvenient)
- feiertage: array of objects - where the objects have the following attributes
    - woche : int - the week (wochen) number in witch the following days will be
    - tage : array of int - all days which are holidays in the given week
- exkursGruppen: array of array of int - these are the groups for the Exkurs courses. Each group is a list of integers representing the numbers which are associated with the students
- curriculumGruppen: array of array of int - like exkursGruppen

# Boundaries
- All input data has to be positive
- Week numbers can only be in a consecutive range
- It will be not checked if the groups are consistent, meaning that students can be members of two separate groups of the same type or be no member of any group at all
- Omitting input keys will result in undefined behavior
- Adding input keys (especially duplicates) will result in undefined behavior
- The distribution is deterministic and should produce the same output for the same input