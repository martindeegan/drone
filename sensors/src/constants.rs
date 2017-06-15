pub const L3G_CTRL_REG1 : u8 = 0x20;
pub const L3G_CTRL_REG2 : u8 = 0x21;
pub const L3G_CTRL_REG3 : u8 = 0x22;
pub const L3G_CTRL_REG4 : u8 =  0x23;
pub const L3G_CTRL_REG5 : u8 =  0x24;
pub const L3G_REFERENCE : u8 =   0x25;
pub const L3G_OUT_TEMP  : u8 =  0x26;
pub const L3G_STATUS_REG : u8 =  0x27;

pub const L3G_OUT_X_L   : u8    =  0x28;
pub const L3G_OUT_X_H   : u8    =  0x29;
pub const L3G_OUT_Y_L  : u8     =  0x2A;
pub const L3G_OUT_Y_H   : u8    =  0x2B;
pub const L3G_OUT_Z_L    : u8   =  0x2C;
pub const L3G_OUT_Z_H    : u8   =  0x2D;

pub const G_GAIN : f32 = 0.070;

// register addresses
pub const MAG_ADDRESS          : u8 =  (0x3C >> 1);
pub const ACC_ADDRESS            : u8 = (0x32 >> 1);
pub const ACC_ADDRESS_SA0_A_LOW  : u8 = (0x30 >> 1);
pub const ACC_ADDRESS_SA0_A_HIGH : u8 = (0x32 >> 1);

pub const LSM303_CTRL_REG1_A   : u8   = 0x20;
pub const LSM303_CTRL_REG2_A   : u8   = 0x21;
pub const LSM303_CTRL_REG3_A   : u8   = 0x22;
pub const LSM303_CTRL_REG4_A   : u8   = 0x23;
pub const LSM303_CTRL_REG5_A   : u8   = 0x24;
pub const LSM303_CTRL_REG6_A   : u8   = 0x25; // DLHC only
pub const LSM303_HP_FILTER_RESET_A : u8= 0x25; // DLH, DLM only
pub const LSM303_REFERENCE_A   : u8   = 0x26;
pub const LSM303_STATUS_REG_A  : u8   = 0x27;

pub const LSM303_OUT_X_L_A     : u8   = 0x28;
pub const LSM303_OUT_X_H_A     : u8   = 0x29;
pub const LSM303_OUT_Y_L_A     : u8   = 0x2A;
pub const LSM303_OUT_Y_H_A     : u8   = 0x2B;
pub const LSM303_OUT_Z_L_A     : u8   = 0x2C;
pub const LSM303_OUT_Z_H_A     : u8   = 0x2D;

pub const LSM303_FIFO_CTRL_REG_A : u8  = 0x2E; // DLHC only
pub const LSM303_FIFO_SRC_REG_A: u8   = 0x2F; // DLHC only

pub const LSM303_INT1_CFG_A    : u8   = 0x30;
pub const LSM303_INT1_SRC_A    : u8   = 0x31;
pub const LSM303_INT1_THS_A    : u8   = 0x32;
pub const LSM303_INT1_DURATION_A : u8  = 0x33;
pub const LSM303_INT2_CFG_A    : u8   = 0x34;
pub const LSM303_INT2_SRC_A    : u8   = 0x35;
pub const LSM303_INT2_THS_A    : u8   = 0x36;
pub const LSM303_INT2_DURATION_A : u8  = 0x37;

pub const LSM303_CLICK_CFG_A   : u8   = 0x38; // DLHC only
pub const LSM303_CLICK_SRC_A   : u8   = 0x39; // DLHC only
pub const LSM303_CLICK_THS_A   : u8   = 0x3A; // DLHC only
pub const LSM303_TIME_LIMIT_A  : u8   = 0x3B; // DLHC only
pub const LSM303_TIME_LATENCY_A: u8   = 0x3C; // DLHC only
pub const LSM303_TIME_WINDOW_A : u8   = 0x3D; // DLHC only

pub const LSM303_CRA_REG_M     : u8   = 0x00;
pub const LSM303_CRB_REG_M     : u8   = 0x01;
pub const LSM303_MR_REG_M      : u8   = 0x02;

pub const LSM303_OUT_X_H_M     : u8   = 0x03;
pub const LSM303_OUT_X_L_M     : u8   = 0x04;

//pub const LSM303_OUT_Y_H_M   : u8   =   -1;   // The addresses of the Y and Z magnetometer output registers
//pub const LSM303_OUT_Y_L_M    : u8   =  -2 ;  // are reversed on the DLM and DLHC relative to the DLH.
//pub const LSM303_OUT_Z_H_M    : u8   =  -3 ;  // These four defines have dummy values so the library can
//pub const LSM303_OUT_Z_L_M    : u8   =  -4 ;  // determine the correct address based on the device type.

pub const LSM303_SR_REG_M      : u8   = 0x09;
pub const LSM303_IRA_REG_M     : u8   = 0x0A;
pub const LSM303_IRB_REG_M     : u8   = 0x0B;
pub const LSM303_IRC_REG_M     : u8   = 0x0C;

pub const LSM303_WHO_AM_I_M    : u8   = 0x0F; // DLM only

pub const LSM303_TEMP_OUT_H_M  : u8   = 0x31; // DLHC only
pub const LSM303_TEMP_OUT_L_M  : u8   = 0x32; // DLHC only
pub const LSM303DLH_OUT_Y_H_M  : u8   = 0x05;
pub const LSM303DLH_OUT_Y_L_M  : u8   = 0x06;
pub const LSM303DLH_OUT_Z_H_M  : u8   = 0x07;
pub const LSM303DLH_OUT_Z_L_M  : u8   = 0x08;

pub const LSM303DLM_OUT_Z_H_M  : u8   = 0x05;
pub const LSM303DLM_OUT_Z_L_M  : u8   = 0x06;
pub const LSM303DLM_OUT_Y_H_M  : u8   = 0x07;
pub const LSM303DLM_OUT_Y_L_M  : u8   = 0x08;

pub const LSM303DLHC_OUT_Z_H_M : u8   = 0x05;
pub const LSM303DLHC_OUT_Z_L_M : u8   = 0x06;
