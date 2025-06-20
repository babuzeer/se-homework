use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{ChargingMode, PileStatus, FAST_CHARGING_POWER, SLOW_CHARGING_POWER};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingPile {
    pub id: Uuid,                    // 充电桩ID
    pub number: String,             // 充电桩编号
    pub mode: ChargingMode,          // 充电模式
    pub status: PileStatus,          // 当前状态
    pub total_charge_count: u32,     // 累计充电次数
    pub total_charge_time: f64,      // 累计充电时长（小时）
    pub total_charge_amount: f64,    // 累计充电量（度）
    pub total_charging_fee: f64,     // 累计充电费用
    pub total_service_fee: f64,      // 累计服务费用
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,  // 启动时间
}

impl ChargingPile {
    pub fn new(number: String, mode: ChargingMode) -> Self {
        Self {
            id: Uuid::new_v4(),
            number,
            mode,
            status: PileStatus::Available,
            total_charge_count: 0,
            total_charge_time: 0.0,
            total_charge_amount: 0.0,
            total_charging_fee: 0.0,
            total_service_fee: 0.0,
            started_at: None,
        }
    }

    pub fn get_power(&self) -> f64 {
        match self.mode {
            ChargingMode::Fast => FAST_CHARGING_POWER,
            ChargingMode::Slow => SLOW_CHARGING_POWER,
        }
    }

    /// 开始充电
    pub fn start_charging(&mut self) -> Result<(), String> {
        match self.status {
            PileStatus::Available => {
                self.status = PileStatus::Charging;
                self.started_at = Some(chrono::Utc::now());
                Ok(())
            }
            _ => Err("充电桩当前状态不可用".to_string()),
        }
    }

    /// 结束充电
    pub fn stop_charging(&mut self, charge_time: f64, charge_amount: f64) -> Result<(), String> {
        match self.status {
            PileStatus::Charging => {
                self.status = PileStatus::Available;
                self.total_charge_count += 1;
                self.total_charge_time += charge_time;
                self.total_charge_amount += charge_amount;
                self.total_charging_fee += charge_amount * self.get_power();
                self.total_service_fee += charge_amount * self.get_power() * charge_time;
                Ok(())
            }
            _ => Err("充电桩不在充电状态".to_string()),
        }
    }

    /// 报告故障
    pub fn report_fault(&mut self) {
        self.status = PileStatus::Fault;
    }

    /// 故障修复
    pub fn repair(&mut self) -> Result<(), String> {
        match self.status {
            PileStatus::Fault => {
                self.status = PileStatus::Available;
                Ok(())
            }
            _ => Err("充电桩不在故障状态".to_string()),
        }
    }

    /// 关机
    pub fn shutdown(&mut self) -> Result<(), String> {
        match self.status {
            PileStatus::Available => {
                self.status = PileStatus::Shutdown;
                Ok(())
            }
            _ => Err("充电桩当前状态不可关机".to_string()),
        }
    }

    /// 开机
    pub fn startup(&mut self) -> Result<(), String> {
        match self.status {
            PileStatus::Shutdown => {
                self.status = PileStatus::Available;
                Ok(())
            }
            _ => Err("充电桩不在关机状态".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pile() {
        let pile = ChargingPile::new("C1".to_string(), ChargingMode::Fast);
        assert_eq!(pile.number, "C1");
        assert_eq!(pile.mode, ChargingMode::Fast);
        assert_eq!(pile.status, PileStatus::Available);
    }

    #[test]
    fn test_charging_cycle() {
        let mut pile = ChargingPile::new("C1".to_string(), ChargingMode::Fast);
        
        // 开始充电
        pile.start_charging().unwrap();
        assert_eq!(pile.status, PileStatus::Charging);
        
        // 结束充电
        pile.stop_charging(2.5, 75.0).unwrap();
        assert_eq!(pile.status, PileStatus::Available);
        assert_eq!(pile.total_charge_count, 1);
        assert_eq!(pile.total_charge_time, 2.5);
        assert_eq!(pile.total_charge_amount, 75.0);
    }

    #[test]
    fn test_fault_handling() {
        let mut pile = ChargingPile::new("C1".to_string(), ChargingMode::Fast);
        
        // 报告故障
        pile.report_fault();
        assert_eq!(pile.status, PileStatus::Fault);
        
        // 修复故障
        pile.repair().unwrap();
        assert_eq!(pile.status, PileStatus::Available);
    }

    #[test]
    fn test_power_management() {
        let mut pile = ChargingPile::new("C1".to_string(), ChargingMode::Fast);
        
        // 关机
        pile.shutdown().unwrap();
        assert_eq!(pile.status, PileStatus::Shutdown);
        
        // 开机
        pile.startup().unwrap();
        assert_eq!(pile.status, PileStatus::Available);
    }
} 