//! # TimeSeries Iterators
use std::cmp;
use std::hash::Hash;

use serde::{Serialize};

use crate::data_elements::TimeSeriesDataPoint;
use crate::timeseries::TimeSeries;

/// An iterator that gaurentees proper ordering of a TimeSeries. if this iterator encounters a non monitonically increasing value it stops evaluating
pub struct OrderedTimeSeriesIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    priorts: Option<&'a TDate>
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> OrderedTimeSeriesIter<'a, TDate, T>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, index: usize) -> OrderedTimeSeriesIter<'a, TDate, T>{
        OrderedTimeSeriesIter {
            ts,
            index,
            priorts: None
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> Iterator for OrderedTimeSeriesIter<'a, TDate, T> {
    type Item = TimeSeriesDataPoint<TDate,T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            let rval = Some(TimeSeriesDataPoint::new(
                self.ts.timeindicies[self.index - 1].clone(),
                self.ts.values[self.index - 1].clone()
            ));
            match self.priorts.is_none() {
                true => {
                    self.priorts = Some(&self.ts.timeindicies[self.index - 1]);
                    rval
                },
                false => match self.ts.timeindicies[self.index - 1] >= *self.priorts.unwrap(){
                    true => {
                        self.priorts = Some(&self.ts.timeindicies[self.index - 1]);
                        rval
                    },
                    false => {
                        None
                    }
                }
            }          
        } else {
            None
        }
    }
}
/// An iterator that gaurentees proper ordering of a TimeSeries. if this iterator encounters a non monitonically increasing value it stops evaluating. It returns its values by reference
pub struct OrderedTimeSeriesRefIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    priorts: Option<&'a TDate>
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> OrderedTimeSeriesRefIter<'a, TDate, T>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, index: usize) -> OrderedTimeSeriesRefIter<'a, TDate, T>{
        OrderedTimeSeriesRefIter {
            ts,
            index,
            priorts: None
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> Iterator for OrderedTimeSeriesRefIter<'a, TDate, T> {
    type Item = TimeSeriesDataPoint<&'a TDate,&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            let rval = Some(TimeSeriesDataPoint::new(
                &self.ts.timeindicies[self.index - 1],
                &self.ts.values[self.index - 1]
            ));
            match self.priorts.is_none() {
                true => {
                    self.priorts = Some(&self.ts.timeindicies[self.index - 1]);
                    rval
                },
                false => match self.ts.timeindicies[self.index - 1] >= *self.priorts.unwrap(){
                    true => {
                        self.priorts = Some(&self.ts.timeindicies[self.index - 1]);
                        rval
                    },
                    false => {
                        None
                    }
                }
            }          
        } else {
            None
        }
    }
}

/// An generic iterator for a timeseries. It does not gaurenteee order. Data is iterated in order of insertion into the containers.
pub struct TimeSeriesIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> TimeSeriesIter<'a, TDate, T>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, index: usize) -> TimeSeriesIter<'a, TDate, T>{
        TimeSeriesIter {
            ts,
            index
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone > Iterator for TimeSeriesIter<'a, TDate, T> {
    type Item = TimeSeriesDataPoint<TDate,T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            Some(TimeSeriesDataPoint::new(
                self.ts.timeindicies[self.index - 1].clone(),
                self.ts.values[self.index - 1].clone()
            ))        
        } else {
            None
        }
    }
}

/// An generic iterator for a timeseries. It does not gaurenteee order. Data is iterated in order of insertion into the containers. Values are returned by Reference
pub struct TimeSeriesRefIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> TimeSeriesRefIter<'a, TDate, T>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, index: usize) -> TimeSeriesRefIter<'a, TDate, T>{
        TimeSeriesRefIter {
            ts,
            index
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone > Iterator for TimeSeriesRefIter<'a, TDate, T> {
    type Item = TimeSeriesDataPoint<&'a TDate,&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            Some(TimeSeriesDataPoint::new(
                &self.ts.timeindicies[self.index - 1],
                &self.ts.values[self.index - 1]
            ))        
        } else {
            None
        }
    }
}


/// A trait the allows you to collect an `Iterator<Item = TimeSeriesDataPoint<TDate,T>>` into a `TimeSeries<TDate,T>` without a potential reorder. Use this method if you know that your data points are in ascending order.
pub trait FromUncheckedIterator<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord,T: Clone>{
    fn collect_from_unchecked_iter(self) -> TimeSeries<TDate,T>;    
}

impl<'a, Tin, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> FromUncheckedIterator<'a, TDate,T> for Tin 
where Tin: Iterator<Item = TimeSeriesDataPoint<TDate,T>> {
    fn collect_from_unchecked_iter(self) -> TimeSeries<TDate,T> {
        TimeSeries::from_tsdatapoints_unchecked(self.into_iter().collect())
    }
}

/// an iterator that represents a shift of the values of the Timeseries vs the index. i.e. a -1 shift index means a lag and +1 means a shift forward
pub struct ShiftedTimeSeriesIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    shift_index: isize
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> ShiftedTimeSeriesIter<'a, TDate, T>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, index: usize, shift: isize) -> ShiftedTimeSeriesIter<'a, TDate, T>{
        let shift_index = -shift;
        let init_index = match shift_index < 0{
            true => (-shift_index) as usize,
            false => index
        };

        ShiftedTimeSeriesIter {
            ts,
            index: init_index,
            shift_index
        }
    }
}

fn add_offset(indexer: usize, delta: isize) -> Option<usize> {
    if delta < 0 {
        indexer.checked_sub(delta.wrapping_abs() as usize)
    } else {
        indexer.checked_add(delta as usize)
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone> Iterator for ShiftedTimeSeriesIter<'a, TDate, T> {
    type Item = TimeSeriesDataPoint<TDate,T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let tsbounds = self.ts.len();
        let timeidx =  add_offset(self.index , self.shift_index - 1);        
        if let Some(tidx) = timeidx 
        {
            let valueidx = self.index - 1;
            let curidx = cmp::max(tidx, valueidx);
            if  curidx < tsbounds {    
                Some(TimeSeriesDataPoint::new(
                    self.ts.timeindicies[tidx].clone(),
                    self.ts.values[valueidx].clone()))
            } else {
                None
            }
        }
        else{
            None
        }       
        
    }
}
/// an iterator that represents a rolling operation on a Timeseries. Data in the window is held in a buffer that gets reduced according the the transform func.
pub struct RollingTimeSeriesIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    transform_func: fn(&Vec<T>)->TReduce,
    buffer: Vec<T>,
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> RollingTimeSeriesIter<'a, TDate, T, TReduce>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, window_size: usize,transform_func: fn(&Vec<T>)->TReduce) -> RollingTimeSeriesIter<'a, TDate, T, TReduce>{
        let init_index = window_size - 1;
        RollingTimeSeriesIter {
            ts,
            index: init_index,
            transform_func,
            buffer: ts.values[0..(window_size-1)].to_vec()
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> Iterator for RollingTimeSeriesIter<'a, TDate, T, TReduce> {
    type Item = TimeSeriesDataPoint<TDate,TReduce>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            let rv = self.ts.values[self.index - 1].clone();
            let func = self.transform_func;            
            self.buffer.push(rv);
            let newv = func(&self.buffer);
            self.buffer.remove(0);
            Some(TimeSeriesDataPoint::new(
                self.ts.timeindicies[self.index - 1].clone(),
                newv
            ))
        } else {
            None
        }
    }
}

/// an iterator that represents a rolling operation on a Timeseries. the transform value is computed according the update and decrement functions, i.e. if you wanted to get the rolling sum you would make it such that update_func => existing value + next value and decrement_func => existing value - last value
pub struct RollingTimeSeriesIterWithUpdate<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T:Clone, TReduce: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    ref_value: Option<TReduce>,
    last_value: &'a T,
    update_func: fn(Option<TReduce>, &T)->Option<TReduce>,
    decrement_func: fn(Option<TReduce>, &T)->Option<TReduce>,
    window_size: usize
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> RollingTimeSeriesIterWithUpdate<'a, TDate, T, TReduce>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, window_size: usize, update_func: fn(Option<TReduce>, &T)->Option<TReduce>, decrement_func: fn(Option<TReduce>, &T)->Option<TReduce>) -> RollingTimeSeriesIterWithUpdate<'a, TDate, T, TReduce>{
        let init_index = window_size - 1;
        let initval = ts.values[0..(window_size)].to_vec().iter().fold(None,update_func);
        RollingTimeSeriesIterWithUpdate {
            ts,
            index: init_index,
            ref_value : initval,
            last_value: &ts.values[window_size-1],
            update_func,
            decrement_func,
            window_size
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> Iterator for RollingTimeSeriesIterWithUpdate<'a, TDate, T, TReduce> {
    type Item = TimeSeriesDataPoint<TDate,TReduce>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index  < self.ts.len() {
            self.index += 1;
            let rv = self.ts.values[self.index - 1].clone();
            let up_func = self.update_func;
            let dec_func = self.decrement_func;
            self.ref_value = up_func(self.ref_value.clone(),&rv);
            self.ref_value = dec_func(self.ref_value.clone(),&self.last_value);
            self.last_value = &self.ts.values[self.index - self.window_size];
            match self.ref_value.is_some() { 
            true => Some(TimeSeriesDataPoint::new(
                self.ts.timeindicies[self.index - 1].clone(),
                self.ref_value.clone().unwrap()
            )),
            false => None
        }
        } else {
            None
        }
    }
}


/// an iterator that represents a skip operation on a Timeseries. Skips take the given span_size and apply a func on the two points on the edges to come up with a new values. You can express difference as a skip operations, e.g. `transform_func = |prior,next| next - prior`
pub struct SkipApplyTimeSeriesIter<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T:Clone, TReduce: Clone> {
    ts: &'a TimeSeries<TDate,T>,
    index: usize,
    span_size: usize,
    transform_func: fn(&T,&T)->TReduce,
    prior_value: T
}

impl<'a, TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone , TReduce: Clone> SkipApplyTimeSeriesIter<'a, TDate, T, TReduce>{
    pub fn new(ts: &'a TimeSeries<TDate,T>, span_size: usize,transform_func: fn(&T,&T)->TReduce) -> SkipApplyTimeSeriesIter<'a, TDate, T, TReduce>{
        let init_index = span_size;
        SkipApplyTimeSeriesIter {
            ts,
            index: init_index,
            span_size,
            transform_func,
            prior_value: ts.values[0].clone()
        }
    }
}

impl<'a,TDate: Serialize + Hash + Clone + cmp::Eq + cmp::Ord, T: Clone, TReduce: Clone> Iterator for SkipApplyTimeSeriesIter<'a, TDate, T, TReduce> {
    type Item = TimeSeriesDataPoint<TDate,TReduce>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index - self.span_size + 1 < self.ts.len() {
            self.index += self.span_size;
            let rv = self.ts.values[self.index - self.span_size].clone();
            let func = self.transform_func;
            let newv = func(&self.prior_value,&rv);
            self.prior_value = rv;
            Some(TimeSeriesDataPoint::new(
                self.ts.timeindicies[self.index - self.span_size].clone(),
                newv
            ))
        } else {
            None
        }
    }
}




/// -----------------------------------------------------------------------------------------------------------------------------------------
/// Unit Test Area
/// -----------------------------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime};
    
    #[test]
    fn test_lag() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        let tslag: TimeSeries<NaiveDateTime,f64> = ts.shift(-1).collect();
        // println!("{:.2?}",tslag);
        // println!("{:.2?}",ts);
        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 1 as i64,0), 1.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 3 as i64,0), 3.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 4 as i64,0), 4.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, tslag);
    }

    #[test]
    fn test_out_of_range_lag() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        let tslag: TimeSeries<NaiveDateTime,f64> = ts.shift(5).collect();
        assert_eq!(tslag.len(), 0);
    }

    #[test]
    fn test_rollfwd() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        let tslag: TimeSeries<NaiveDateTime,f64> = ts.shift(1).collect();
        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 0 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 1 as i64,0), 3.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 4.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 3 as i64,0), 5.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, tslag);
    }

    #[test]
    fn test_rolling() {
        let values = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        
        fn roll_func(buffer: &Vec<f64>) -> f64{
            buffer.iter().sum()
        };

        let tsrolled: TimeSeries<NaiveDateTime,f64> = ts.apply_rolling(2, roll_func).collect();
        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 1 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 3 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 4 as i64,0), 2.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, tsrolled);
    }

    #[test]
    fn test_rolling_with_update() {
        let values = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        
        fn update(prior: Option<f64>, next: &f64) -> Option<f64>{
            let v =  match prior.is_some(){
                true => prior.unwrap(),
                false => 0.0
            };
            Some(v + next)
        };

        fn decrement(next: Option<f64>, prior: &f64) -> Option<f64>{
            let v =  match next.is_some(){
                true => next.unwrap(),
                false => 0.0
            };
            Some(v - prior)
        };

        let tsrolled: TimeSeries<NaiveDateTime,f64> = ts.apply_updating_rolling(2, update, decrement).collect();
        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 1 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 3 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 4 as i64,0), 2.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, tsrolled);
    }

    #[test]
    fn test_rolling_issame() {
        let values = vec![1.0, 4.0, 2.0, 9.0, 100.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        
        fn roll_func(buffer: &Vec<f64>) -> f64{
            buffer.iter().sum()
        };

        let buffered: TimeSeries<NaiveDateTime,f64> = ts.apply_rolling(2, roll_func).collect();

        fn update(prior: Option<f64>, next: &f64) -> Option<f64>{
            let v =  match prior.is_some(){
                true => prior.unwrap(),
                false => 0.0
            };
            Some(v + next)
        };

        fn decrement(next: Option<f64>, prior: &f64) -> Option<f64>{
            let v =  match next.is_some(){
                true => next.unwrap(),
                false => 0.0
            };
            Some(v - prior)
        };

        let updated: TimeSeries<NaiveDateTime,f64> = ts.apply_updating_rolling(2, update, decrement).collect();
        assert_eq!(buffered, updated);

    }

    #[test]
    fn test_skip() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        
        fn change_func(prior: &f64, curr: &f64) -> f64{
            curr - prior
        };

        let ts_skipped: TimeSeries<NaiveDateTime,f64> = ts.skip_apply(1, change_func).collect();

        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 1 as i64,0), 1.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 1.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 3 as i64,0), 1.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 4 as i64,0), 1.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, ts_skipped);
    }

    #[test]
    fn test_skip_2span() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let index = (0..values.len()).map(|i| NaiveDateTime::from_timestamp(60 * i as i64,0)).collect();
        let ts = TimeSeries::from_vecs(index, values).unwrap();
        
        fn change_func(prior: &f64, curr: &f64) -> f64{
            curr - prior
        };

        let ts_skipped: TimeSeries<NaiveDateTime,f64> = ts.skip_apply(2, change_func).collect();

        let data = vec![
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 2 as i64,0), 2.0),
            TimeSeriesDataPoint::new(NaiveDateTime::from_timestamp(60 * 4 as i64,0), 2.0),
        ];
        let tsexp = TimeSeries::from_tsdatapoints(data).unwrap();
        assert_eq!(tsexp, ts_skipped);
    }

}