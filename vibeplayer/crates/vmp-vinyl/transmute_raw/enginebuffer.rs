//! Transmuted by v01d (FUTE) — C/C++ → Rust
//! Backend: libclang AST (main-file only)
//! Origin: ../mixxx/src/engine/enginebuffer.h

#![allow(dead_code, non_snake_case, unused_variables)]

#[derive(Debug, Clone)]
pub struct EngineBuffer {
    pub field_0: i32,
    pub field_1: i32,
    pub field_2: i32,
    pub m_group: i32,
    pub m_channelIndex: i32,
    pub m_pConfig: i32,
    pub m_pLoopingControl: &mut LoopingControl,
    pub m_pEngineSync: &mut EngineSync,
    pub m_pSyncControl: &mut SyncControl,
    pub m_pVinylControlControl: &mut VinylControlControl,
    pub m_pRateControl: &mut RateControl,
    pub m_pBpmControl: &mut BpmControl,
    pub m_pKeyControl: &mut KeyControl,
    pub m_pClockControl: &mut ClockControl,
    pub m_pCueControl: &mut CueControl,
    pub m_engineControls: i32,
    pub m_pReadAheadManager: &mut ReadAheadManager,
    pub m_pReader: &mut i32,
    pub m_hintList: i32,
    pub m_playPos: i32,
    pub m_speed_old: f64,
    pub m_actual_speed: f64,
    pub m_tempo_ratio_old: f64,
    pub m_scratching_old: bool,
    pub m_reverse_old: bool,
    pub m_pitch_old: f64,
    pub m_baserate_old: f64,
    pub m_rate_old: f64,
    pub m_trackEndPositionOld: i32,
    pub m_trackSampleRateOld: i32,
    pub m_pause: i32,
    pub m_samplesSinceLastIndicatorUpdate: Size,
    pub m_slipPos: i32,
    pub m_dSlipRate: f64,
    pub m_bSlipEnabledProcessing: bool,
    pub m_slipModeState: i32,
    pub m_pTrackSamples: &mut ControlObject,
    pub m_pTrackSampleRate: &mut ControlObject,
    pub m_playButton: &mut ControlPushButton,
    pub m_playStartButton: &mut ControlPushButton,
    pub m_stopStartButton: &mut ControlPushButton,
    pub m_stopButton: &mut ControlPushButton,
    pub m_pSlipButton: &mut ControlPushButton,
    pub m_quantize: i32,
    pub m_playposSlider: &mut ControlPotmeter,
    pub m_pSampleRate: &mut ControlProxy,
    pub m_pKeylockEngine: &mut ControlProxy,
    pub m_pKeylock: &mut ControlPushButton,
    pub m_pReplayGain: &mut ControlProxy,
    pub m_pPassthroughEnabled: &mut ControlProxy,
    pub m_pTrackLoaded: &mut ControlObject,
    pub m_pRepeat: &mut ControlPushButton,
    pub m_startButton: &mut ControlPushButton,
    pub m_endButton: &mut ControlPushButton,
    pub m_pScale: &mut EngineBufferScale,
    pub m_pScaleVinyl: &mut EngineBufferScale,
    pub m_pScaleKeylock: &mut EngineBufferScale,
    pub m_pScaleLinear: &mut EngineBufferScaleLinear,
    pub m_pScaleST: &mut EngineBufferScaleST,
    pub m_bScalerChanged: bool,
    pub m_bScalerOverride: bool,
    pub m_iSeekPhaseQueued: i32,
    pub m_iEnableSyncQueued: i32,
    pub m_iSyncModeQueued: i32,
    pub m_queuedSeek: i32,
    pub m_previousBufferSeek: bool,
    pub m_slipQuitAndAdopt: i32,
    pub m_pChannelToCloneFrom: i32,
    pub m_iTrackLoading: i32,
    pub m_bPlayAfterLoading: bool,
    pub m_sampleRate: i32,
    pub m_channelCount: i32,
    pub m_pCurrentTrack: i32,
    pub m_pCrossfadeBuffer: &mut i32,
    pub m_bCrossfadeReady: bool,
    pub m_lastBufferSize: Size,
    pub m_visualPlayPos: i32,
}

#[derive(Debug, Clone)]
pub struct QueuedSeek {
    pub position: i32,
    pub seekType: Enum SeekRequest,
}

pub fn Q_DECLARE_FLAGS(arg0: i32, arg1: SeekRequest) -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn EngineBuffer(group: Int &, pConfig: i32, pChannel: &mut EngineChannel, pMixingEngine: &mut EngineMixer, maxSupportedChannel: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn bindWorkers(pWorkerScheduler: &mut EngineWorkerScheduler) {
    todo!("FUTE scaffold — polish body")
}

pub fn getGroup() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn getSpeed() -> f64 {
    todo!("FUTE scaffold — polish body")
}

pub fn getChannelCount() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn getPlayPos() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn getScratching() -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn isReverse() -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn getBpm() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn getLocalBpm() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn setBeatLoop(startPosition: i32, enabled: bool) {
    todo!("FUTE scaffold — polish body")
}

pub fn setLoop(startPosition: i32, endPositon: i32, enabled: bool) {
    todo!("FUTE scaffold — polish body")
}

pub fn setEngineMixer(arg0: &mut EngineMixer) {
    todo!("FUTE scaffold — polish body")
}

pub fn queueNewPlaypos(newpos: i32, seekType: Enum SeekRequest) {
    todo!("FUTE scaffold — polish body")
}

pub fn requestSyncPhase() {
    todo!("FUTE scaffold — polish body")
}

pub fn requestEnableSync(enabled: bool) {
    todo!("FUTE scaffold — polish body")
}

pub fn requestSyncMode(mode: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn process(pOut: &mut i32, bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn processSlip(bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn postProcessLocalBpm() {
    todo!("FUTE scaffold — polish body")
}

pub fn postProcess(bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn queuedSeekPosition() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn isTrackLoaded() -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn getLoadedTrack() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn ejectTrack() {
    todo!("FUTE scaffold — polish body")
}

pub fn getExactPlayPos() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn getVisualPlayPos() -> f64 {
    todo!("FUTE scaffold — polish body")
}

pub fn getTrackEndPosition() -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn setTrackEndPosition(position: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn getUserOffset() -> f64 {
    todo!("FUTE scaffold — polish body")
}

pub fn getRateRatio() -> f64 {
    todo!("FUTE scaffold — polish body")
}

pub fn collectFeatures(pGroupFeatures: &mut i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn setScalerForTest(pScaleVinyl: &mut EngineBufferScale, pScaleKeylock: &mut EngineBufferScale) {
    todo!("FUTE scaffold — polish body")
}

pub fn loadFakeTrack(pTrack: i32, bPlay: bool) {
    todo!("FUTE scaffold — polish body")
}

pub fn getKeylockEngineName(engine: KeylockEngine) -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn isKeylockEngineAvailable(engine: KeylockEngine) -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn defaultKeylockEngine() -> KeylockEngine {
    todo!("FUTE scaffold — polish body")
}

pub fn loadTrack(pTrack: i32, play: bool, pChannelToCloneFrom: &mut EngineChannel) {
    todo!("FUTE scaffold — polish body")
}

pub fn setChannelIndex(channelIndex: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn seekAbs(arg0: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn seekExact(arg0: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn verifyPlay() {
    todo!("FUTE scaffold — polish body")
}

pub fn slipQuitAndAdopt() {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlPlayFromStart(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlJumpToStartAndStop(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlStop(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlStart(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlEnd(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotControlSeek(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotKeylockEngineChanged(arg0: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn trackLoadFailed(pTrack: i32, reason: Int &) {
    todo!("FUTE scaffold — polish body")
}

pub fn noVinylControlInputConfigured() {
    todo!("FUTE scaffold — polish body")
}

pub fn slotTrackLoaded(pTrack: i32, trackSampleRate: i32, trackChannelCount: i32, trackNumFrame: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotTrackLoadFailed(pTrack: i32, reason: Int &) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotPassthroughChanged(v: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn slotUpdatedTrackBeats() {
    todo!("FUTE scaffold — polish body")
}

pub fn addControl(pControl: &mut EngineControl) {
    todo!("FUTE scaffold — polish body")
}

pub fn enableIndependentPitchTempoScaling(bEnable: bool, bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn updateIndicators(rate: f64, bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn hintReader(rate: f64) {
    todo!("FUTE scaffold — polish body")
}

pub fn fractionalPlayposFromAbsolute(position: i32) -> f64 {
    todo!("FUTE scaffold — polish body")
}

pub fn doSeekFractional(fractionalPos: f64, seekType: Enum SeekRequest) {
    todo!("FUTE scaffold — polish body")
}

pub fn doSeekPlayPos(position: i32, seekType: Enum SeekRequest) {
    todo!("FUTE scaffold — polish body")
}

pub fn readToCrossfadeBuffer(bufferSize: Size) {
    todo!("FUTE scaffold — polish body")
}

pub fn setNewPlaypos(playpos: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn processSyncRequests() {
    todo!("FUTE scaffold — polish body")
}

pub fn processSeek(paused: bool) {
    todo!("FUTE scaffold — polish body")
}

pub fn FRIEND_TEST(arg0: i32, arg1: i32) -> i32 {
    todo!("FUTE scaffold — polish body")
}

pub fn previousBufferSeek() -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn updateIndicatorsAndModifyPlay(newPlay: bool, oldPlay: bool) -> bool {
    todo!("FUTE scaffold — polish body")
}

pub fn notifyTrackLoaded(pNewTrack: i32, pOldTrack: i32) {
    todo!("FUTE scaffold — polish body")
}

pub fn processTrackLocked(pOutput: &mut i32, bufferSize: Size, sampleRate: i32) {
    todo!("FUTE scaffold — polish body")
}

