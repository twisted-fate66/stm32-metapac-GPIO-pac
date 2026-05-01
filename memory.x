/* memory.x - add FLASH and RAM regions
    Adjust ORIGIN and LENGTH to match your MCU */
MEMORY
{
  FLASH  : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}