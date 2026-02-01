EXTERN(_RESOURCE_TABLE);

SECTIONS
{
    .resource_table : ALIGN(4)
    {
        KEEP(*(.resource_table .resource_table.*));
    }
} INSERT BEFORE .data;
