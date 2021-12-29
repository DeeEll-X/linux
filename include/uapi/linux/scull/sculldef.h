/*
 * scull.h -- definitions for the char module
 *
 * Copyright (C) 2001 Alessandro Rubini and Jonathan Corbet
 * Copyright (C) 2001 O'Reilly & Associates
 *
 * The source code in this file can be freely used, adapted,
 * and redistributed in source or binary form, so long as an
 * acknowledgment appears in derived source files.  The citation
 * should list that the code comes from the book "Linux Device
 * Drivers" by Alessandro Rubini and Jonathan Corbet, published
 * by O'Reilly & Associates.   No warranty is attached;
 * we cannot take responsibility for errors or fitness for use.
 *
 * $Id: scull.h,v 1.15 2004/11/04 17:51:18 rubini Exp $
 */

#ifndef _UAPI_LINUX_SCULLDEF_H_
#define _UAPI_LINUX_SCULLDEF_H_

#include <linux/ioctl.h> /* needed for the _IOW etc stuff used later */

#ifndef SCULL_MAJOR
#define SCULL_MAJOR 0   /* dynamic major by default */
#endif

#ifndef SCULL_NR_DEVS
#define SCULL_NR_DEVS 4    /* scull0 through scull3 */
#endif

#ifndef SCULL_P_NR_DEVS
#define SCULL_P_NR_DEVS 4  /* scullpipe0 through scullpipe3 */
#endif

/*
 * The bare device is a variable-length region of memory.
 * Use a linked list of indirect blocks.
 *
 * "scull_dev->data" points to an array of pointers, each
 * pointer refers to a memory area of SCULL_QUANTUM bytes.
 *
 * The array (quantum-set) is SCULL_QSET long.
 */
#ifndef SCULL_QUANTUM
#define SCULL_QUANTUM 4000
#endif

#ifndef SCULL_QSET
#define SCULL_QSET    1000
#endif

/*
 * The pipe device is a simple circular buffer. Here its default size
 */
#ifndef SCULL_P_BUFFER
#define SCULL_P_BUFFER 4000
#endif

/*
 * Ioctl definitions
 */

/* Use 'k' as magic number */
#define SCULL_IOC_MAGIC  'k'


enum {
    SCULL_IOCRESET    = _IO(SCULL_IOC_MAGIC,   0),
    /*
     * S means "Set" through a ptr,
     * T means "Tell" directly with the argument value
     * G means "Get": reply by setting through a pointer
     * Q means "Query": response is on the return value
     * X means "eXchange": switch G and S atomically
     * H means "sHift": switch T and Q atomically
     */
    SCULL_IOCSQUANTUM = _IOW(SCULL_IOC_MAGIC,  1, int),
    SCULL_IOCSQSET    = _IOW(SCULL_IOC_MAGIC,  2, int),
    SCULL_IOCTQUANTUM = _IO(SCULL_IOC_MAGIC,   3),
    SCULL_IOCTQSET    = _IO(SCULL_IOC_MAGIC,   4),
    SCULL_IOCGQUANTUM = _IOR(SCULL_IOC_MAGIC,  5, int),
    SCULL_IOCGQSET    = _IOR(SCULL_IOC_MAGIC,  6, int),
    SCULL_IOCQQUANTUM = _IO(SCULL_IOC_MAGIC,   7),
    SCULL_IOCQQSET    = _IO(SCULL_IOC_MAGIC,   8),
    SCULL_IOCXQUANTUM = _IOWR(SCULL_IOC_MAGIC, 9, int),
    SCULL_IOCXQSET    = _IOWR(SCULL_IOC_MAGIC,10, int),
    SCULL_IOCHQUANTUM = _IO(SCULL_IOC_MAGIC,  11),
    SCULL_IOCHQSET    = _IO(SCULL_IOC_MAGIC,  12),
    /*
     * The other entities only have "Tell" and "Query", because they're
     * not printed in the book, and there's no need to have all six.
     * (The previous stuff was only there to show different ways to do it.
     */
    SCULL_P_IOCTSIZE  = _IO(SCULL_IOC_MAGIC,   13),
    SCULL_P_IOCQSIZE  = _IO(SCULL_IOC_MAGIC,   14),
};


/* ... more to come */

#define SCULL_IOC_MAXNR 14

#endif /* _UAPI_LINUX_SCULLDEF_H_ */
