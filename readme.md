## Version 2 results on server
 - Memory usuage by program around 5GiB
 - Initial entry speeds spike to 3000-4000 per seconds but then eventually drop to 300-400 due to high data volume around 110000 entries due to high comparisons
 - CPU Cores usuage -  5-6 cores at 0 %
 - Table Size 8.5 MiB
 - Currently store in table tanmaydaga.TANMAY_patch_embed_vit_base_patch16_224
 - Traffic - 190 KB/s (from DB Status)
 - Connections - 26 to SQL Server (IDLE STATE - 4)


 ## Version 3 results on server

    - 14000-15000 insertions per second
    - cpu cores all used with low percentage
    - For 2289665 entries 
    - Query to read data used 
    - 
```sql

-- Query use to read result

SELECT
    rounded_result,
    operand1,
    operand2,
    SUM(original_count) AS total_occurrences
FROM (
    SELECT
        ROUND(result, 7) AS rounded_result,
        LEAST(ROUND(number_a, 7), ROUND(number_b, 7)) AS operand1,
        GREATEST(ROUND(number_a, 7), ROUND(number_b, 7)) AS operand2,
        `count` AS original_count
    FROM
        tanmaydaga.TANMAY_Conv2d_vit_base_patch16_224
) AS pre_aggregated_calculations
GROUP BY
    rounded_result,
    operand1,
    operand2
ORDER BY
    total_occurrences DESC,
    operand2 DESC
LIMIT 10000;
```

    - Results
        <details>
            <summary>Click to view data</summary>

                | Column 1 | Column 2 | Column 3 | Column 4 |
                |---|---|---|---|
                | -0.6541175 | -0.0005649 | 0.0008635 | 15 |
                | 0.8004202 | -0.0027843 | -0.0022286 | 14 |
                | -0.7821732 | -0.0043271 | 0.0055322 | 13 |
                | 0.5553222 | 0.0022976 | 0.0041374 | 13 |
                | -0.6366884 | -0.0025521 | 0.0040084 | 13 |
                | -0.9329847 | -0.0022423 | 0.0020920 | 13 |
                | -0.7412636 | -0.0012102 | 0.0016327 | 13 |
                | -0.7064052 | -0.0018120 | 0.0012800 | 13 |
                | -0.9329847 | -0.0004232 | 0.0003948 | 13 |
                | -0.6715468 | -0.0059342 | 0.0039851 | 12 |
                | -0.7821732 | -0.0044041 | 0.0034447 | 12 |
                | -0.6965494 | -0.0008004 | 0.0011491 | 12 |
                | -1.1931673 | -0.0009448 | 0.0011273 | 12 |
                | -0.7586928 | -0.0006552 | 0.0008635 | 12 |
                | -0.7064052 | -0.0009189 | 0.0006491 | 12 |
                | 0.5553222 | 0.0003106 | 0.0005593 | 12 |
                | -0.8109804 | -0.0215700 | 0.0265974 | 11 |
                | -0.7412636 | -0.0346591 | 0.0256915 | 11 |
                | -0.7412636 | -0.0108483 | 0.0146349 | 11 |
                | -0.7412636 | -0.0078360 | 0.0105712 | 11 |
                | -0.8109804 | -0.0070767 | 0.0057391 | 11 |
                | -0.7412636 | -0.0075306 | 0.0055822 | 11 |
                | -1.1931673 | -0.0065484 | 0.0054883 | 11 |
                | -0.8284096 | -0.0057039 | 0.0047252 | 11 |
                | -0.7412636 | -0.0063605 | 0.0047148 | 11 |
                | -0.7238344 | -0.0025068 | 0.0034633 | 11 |
                | -0.6715468 | -0.0049692 | 0.0033370 | 11 |
                | -0.7412636 | -0.0031690 | 0.0023490 | 11 |
                | -0.9155555 | -0.0025329 | 0.0023190 | 11 |
                | -0.7412636 | -0.0028348 | 0.0021013 | 11 |
                | -0.7412636 | -0.0027279 | 0.0020221 | 11 |
                | 0.8179272 | 0.0015701 | 0.0019196 | 11 |
                | -0.8109804 | -0.0018753 | 0.0015208 | 11 |
                | -0.7412636 | -0.0017886 | 0.0013258 | 11 |
                | -0.8284096 | -0.0015730 | 0.0013031 | 11 |
                | -0.6715468 | -0.0018788 | 0.0012617 | 11 |
                | -0.6541175 | -0.0007589 | 0.0011602 | 11 |
                | -0.7412636 | -0.0008027 | 0.0010828 | 11 |
                | -0.7064052 | -0.0014054 | 0.0009927 | 11 |
                | -1.1760426 | -0.0008069 | 0.0009489 | 11 |
                | -0.8632680 | -0.0007455 | 0.0008635 | 11 |
                | -0.7412636 | -0.0004429 | 0.0005975 | 11 |
                | -0.8458387 | -0.0005483 | 0.0004638 | 11 |
                | -0.7412636 | -0.0002750 | 0.0003709 | 11 |
                | -0.9155555 | -0.0003791 | 0.0003471 | 11 |
                | -1.1931673 | -0.0004093 | 0.0003431 | 11 |

                </details>
