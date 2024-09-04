// SPDX-License-Identifier: GPL-2.0
/*
 * TDX guest user interface driver
 *
 * Copyright (C) 2022 Intel Corporation
 */

#include <linux/kernel.h>
#include <linux/miscdevice.h>
#include <linux/mm.h>
#include <linux/module.h>
#include <linux/mod_devicetable.h>
#include <linux/string.h>
#include <linux/uaccess.h>
#include <linux/set_memory.h>
#include <linux/io.h>
#include <linux/delay.h>
#include <linux/sizes.h>

#include <uapi/linux/tdx-guest.h>

#include <asm/cpu_device_id.h>
#include <asm/tdx.h>


static long tdx_get_report0(struct tdx_report_req __user *req)
{
	u8 *reportdata, *tdreport;
	long ret;

	reportdata = kmalloc(TDX_REPORTDATA_LEN, GFP_KERNEL);
	if (!reportdata)
		return -ENOMEM;

	tdreport = kzalloc(TDX_REPORT_LEN, GFP_KERNEL);
	if (!tdreport)
	{
		ret = -ENOMEM;
		goto out;
	}

	if (copy_from_user(reportdata, req->reportdata, TDX_REPORTDATA_LEN))
	{
		ret = -EFAULT;
		goto out;
	}

	/* Generate TDREPORT0 using "TDG.MR.REPORT" TDCALL */
	ret = tdx_mcall_get_report0(reportdata, tdreport);
	if (ret)
		goto out;

	if (copy_to_user(req->tdreport, tdreport, TDX_REPORT_LEN))
		ret = -EFAULT;

out:
	kfree(reportdata);
	kfree(tdreport);

	return ret;
}

#define TDX_CMD_EXTEND_RTMR _IOR('T', 3, struct tdx_extend_rtmr_req)
#define TDG_MR_RTMR_EXTEND 2
#define TDX_EXTEND_RTMR_DATA_LEN 48
struct tdx_extend_rtmr_req
{
	u8 data[TDX_EXTEND_RTMR_DATA_LEN];
	u8 index;
};

static long tdx_extend_rtmr(struct tdx_extend_rtmr_req __user *req)
{
	u8 *data;
	u8 index;
	long ret;

	data = kmalloc(TDX_EXTEND_RTMR_DATA_LEN, GFP_KERNEL);
	if (!data)
		return -ENOMEM;

	if (copy_from_user(data, req->data, TDX_EXTEND_RTMR_DATA_LEN))
	{
		ret = -EFAULT;
		goto out;
	}

	if (copy_from_user(&index, (u8 __user *)&req->index, 1))
	{
		ret = -EFAULT;
		goto out;
	}

	{
		struct tdx_module_args args = {
			.rcx = virt_to_phys(data),
			.rdx = index,
		};

		ret = __tdcall(TDG_MR_RTMR_EXTEND, &args);
	}
out:
	kfree(data);
	return ret;
}

static long tdx_guest_ioctl(struct file *file, unsigned int cmd,
							unsigned long arg)
{
	switch (cmd)
	{
	case TDX_CMD_GET_REPORT0:
		return tdx_get_report0((struct tdx_report_req __user *)arg);
	case TDX_CMD_EXTEND_RTMR:
		return tdx_extend_rtmr((struct tdx_extend_rtmr_req __user *)arg);
	default:
		return -ENOTTY;
	}
}

static const struct file_operations tdx_guest_fops = {
	.owner = THIS_MODULE,
	.unlocked_ioctl = tdx_guest_ioctl,
	.llseek = no_llseek,
};

static struct miscdevice tdx_misc_dev = {
	.name = KBUILD_MODNAME,
	.minor = MISC_DYNAMIC_MINOR,
	.fops = &tdx_guest_fops,
};

static const struct x86_cpu_id tdx_guest_ids[] = {
	X86_MATCH_FEATURE(X86_FEATURE_TDX_GUEST, NULL),
	{}
};
MODULE_DEVICE_TABLE(x86cpu, tdx_guest_ids);

static int __init tdx_guest_init(void)
{
	int ret;

	if (!x86_match_cpu(tdx_guest_ids))
		return -ENODEV;

	ret = misc_register(&tdx_misc_dev);
	if (ret)
		return ret;


	return 0;

	misc_deregister(&tdx_misc_dev);

	return ret;
}
module_init(tdx_guest_init);

static void __exit tdx_guest_exit(void)
{
	misc_deregister(&tdx_misc_dev);
}
module_exit(tdx_guest_exit);

MODULE_AUTHOR("Kuppuswamy Sathyanarayanan <sathyanarayanan.kuppuswamy@linux.intel.com>");
MODULE_DESCRIPTION("TDX Guest Driver");
MODULE_LICENSE("GPL");
