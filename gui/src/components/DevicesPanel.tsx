import React, { useState, useEffect } from 'react';
import { Music, Wifi, Plus, Trash2, Hash, Check, X, Play, Square } from 'lucide-react';
import { createBuboClient } from '../client';
import type { DeviceInfo, ClientMessage, ServerMessage } from '../types';

const client = createBuboClient();

interface DevicesState {
  devices: DeviceInfo[];
  activeTab: 'midi' | 'osc';
  selectedMidiIndex: number;
  selectedOscIndex: number;
  isConnected: boolean;
  
  // Input modes
  isCreatingVirtualMidi: boolean;
  isCreatingOsc: boolean;
  editingDeviceName: string | null;
  
  // Input values
  slotEditValue: string;
  virtualMidiInput: string;
  oscStep: number;
  oscName: string;
  oscIp: string;
  oscPort: string;
  
  // UI state
  statusMessage: string;
  confirmationDialog: {
    message: string;
    onConfirm: () => void;
    onCancel: () => void;
  } | null;
}

export const DevicesPanel: React.FC = () => {
  const [state, setState] = useState<DevicesState>({
    devices: [],
    activeTab: 'midi',
    selectedMidiIndex: 0,
    selectedOscIndex: 0,
    isConnected: false,
    
    isCreatingVirtualMidi: false,
    isCreatingOsc: false,
    editingDeviceName: null,
    
    slotEditValue: '',
    virtualMidiInput: '',
    oscStep: 0,
    oscName: '',
    oscIp: '',
    oscPort: '',
    
    statusMessage: '',
    confirmationDialog: null,
  });

  const filteredDevices = state.devices
    .filter(device => {
      if (state.activeTab === 'midi') {
        return device.kind === 'Midi';
      } else {
        return device.kind === 'Osc';
      }
    })
    .sort((a, b) => a.name.localeCompare(b.name));

  const currentSelectedIndex = state.activeTab === 'midi' ? state.selectedMidiIndex : state.selectedOscIndex;
  const validSelectedIndex = Math.min(currentSelectedIndex, filteredDevices.length - 1);
  const selectedDevice = filteredDevices[validSelectedIndex >= 0 ? validSelectedIndex : 0];
  
  // Debug logging
  console.log('Current state:', {
    activeTab: state.activeTab,
    totalDevices: state.devices.length,
    filteredDevices: filteredDevices.length,
    currentSelectedIndex,
    validSelectedIndex,
    selectedDevice: selectedDevice?.name
  });

  useEffect(() => {
    checkConnection();
    
    const unsubscribe = client.onMessage((message: ServerMessage) => {
      if ('Hello' in message) {
        console.log('Hello message devices:', message.Hello.devices);
        setState(prev => ({ 
          ...prev, 
          devices: message.Hello.devices,
          selectedMidiIndex: 0,
          selectedOscIndex: 0
        }));
      } else if ('DeviceList' in message) {
        console.log('DeviceList message devices:', message.DeviceList);
        setState(prev => ({ 
          ...prev, 
          devices: message.DeviceList,
          selectedMidiIndex: 0,
          selectedOscIndex: 0
        }));
      } else if (message === 'Success') {
        setState(prev => ({ ...prev, statusMessage: 'Operation successful' }));
        requestDeviceList();
      } else if ('InternalError' in message) {
        setState(prev => ({ ...prev, statusMessage: `Error: ${message.InternalError}` }));
      }
    });

    return () => unsubscribe();
  }, []);

  const checkConnection = async () => {
    try {
      const connected = await client.isConnected();
      setState(prev => ({ ...prev, isConnected: connected }));
      if (connected) {
        requestDeviceList();
      }
    } catch (error) {
      console.error('Failed to check connection:', error);
    }
  };

  const requestDeviceList = async () => {
    try {
      await client.sendMessage("RequestDeviceList");
    } catch (error) {
      console.error('Failed to request device list:', error);
    }
  };

  const sendClientMessage = async (message: ClientMessage) => {
    try {
      await client.sendMessage(message);
    } catch (error) {
      console.error('Failed to send message:', error);
      setState(prev => ({ ...prev, statusMessage: 'Failed to send message' }));
    }
  };

  const handleDeviceConnect = (device: DeviceInfo) => {
    if (device.is_connected) {
      sendClientMessage({ DisconnectMidiDeviceByName: device.name });
    } else {
      sendClientMessage({ ConnectMidiDeviceByName: device.name });
    }
  };

  const handleSlotClick = (device: DeviceInfo) => {
    setState(prev => ({ 
      ...prev, 
      editingDeviceName: device.name,
      slotEditValue: device.id === 0 ? '' : device.id.toString()
    }));
  };

  const handleSlotEditKeyDown = (e: React.KeyboardEvent, device: DeviceInfo) => {
    if (e.key === 'Enter') {
      confirmSlotEdit(device);
    } else if (e.key === 'Escape') {
      setState(prev => ({ ...prev, editingDeviceName: null }));
    }
  };

  const confirmSlotEdit = (device: DeviceInfo) => {
    const slotNum = parseInt(state.slotEditValue);
    
    if (state.slotEditValue === '' || slotNum === 0) {
      sendClientMessage({ UnassignDeviceFromSlot: device.id });
    } else if (isNaN(slotNum) || slotNum < 1 || slotNum > 16) {
      setState(prev => ({ ...prev, statusMessage: 'Invalid slot number (1-16)' }));
      return;
    } else {
      sendClientMessage({ AssignDeviceToSlot: [slotNum, device.name] });
    }
    
    setState(prev => ({ ...prev, editingDeviceName: null }));
  };

  const handleCreateVirtualMidi = () => {
    setState(prev => ({ ...prev, isCreatingVirtualMidi: true, virtualMidiInput: '' }));
  };

  const confirmVirtualMidiCreation = () => {
    if (!state.virtualMidiInput.trim()) {
      setState(prev => ({ ...prev, statusMessage: 'Please enter a port name' }));
      return;
    }
    
    sendClientMessage({ CreateVirtualMidiOutput: state.virtualMidiInput.trim() });
    setState(prev => ({ ...prev, isCreatingVirtualMidi: false, virtualMidiInput: '' }));
  };

  const handleCreateOsc = () => {
    setState(prev => ({ 
      ...prev, 
      isCreatingOsc: true, 
      oscStep: 0, 
      oscName: '', 
      oscIp: '127.0.0.1', 
      oscPort: '57120' 
    }));
  };

  const handleOscStepNext = () => {
    if (state.oscStep === 0 && !state.oscName.trim()) {
      setState(prev => ({ ...prev, statusMessage: 'Please enter a name' }));
      return;
    }
    if (state.oscStep === 1 && !state.oscIp.trim()) {
      setState(prev => ({ ...prev, statusMessage: 'Please enter an IP address' }));
      return;
    }
    if (state.oscStep === 2) {
      const port = parseInt(state.oscPort);
      if (isNaN(port) || port < 1 || port > 65535) {
        setState(prev => ({ ...prev, statusMessage: 'Invalid port number' }));
        return;
      }
      
      sendClientMessage({ CreateOscDevice: [state.oscName.trim(), state.oscIp.trim(), port] });
      setState(prev => ({ ...prev, isCreatingOsc: false }));
      return;
    }
    
    setState(prev => ({ ...prev, oscStep: prev.oscStep + 1 }));
  };

  const handleOscStepBack = () => {
    if (state.oscStep === 0) {
      setState(prev => ({ ...prev, isCreatingOsc: false }));
    } else {
      setState(prev => ({ ...prev, oscStep: prev.oscStep - 1 }));
    }
  };

  const handleRemoveOsc = (device: DeviceInfo) => {
    if (device.kind !== 'Osc') return;
    
    setState(prev => ({
      ...prev,
      confirmationDialog: {
        message: `Remove OSC device ${device.name}?`,
        onConfirm: () => {
          sendClientMessage({ RemoveOscDevice: device.name });
          setState(prev => ({ ...prev, confirmationDialog: null }));
        },
        onCancel: () => setState(prev => ({ ...prev, confirmationDialog: null })),
      },
    }));
  };

  const handleKeyDown = (e: React.KeyboardEvent, action: () => void) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      action();
    }
  };

  const renderDeviceTable = () => {
    const isMidiTab = state.activeTab === 'midi';
    
    return (
      <div className="overflow-hidden">
        <div className="grid grid-cols-4 gap-2 p-2 border-b font-medium text-sm"
             style={{ borderColor: 'var(--color-border)', color: 'var(--color-text)' }}>
          <div>Slot</div>
          <div>Status</div>
          <div>Name</div>
          <div>{isMidiTab ? 'Type' : 'Address'}</div>
        </div>
        
        <div className="max-h-64 overflow-y-auto">
          {filteredDevices.map((device, index) => {
            const isSelected = index === validSelectedIndex;
            const isEditingSlot = state.editingDeviceName === device.name;
            
            return (
              <div
                key={device.name}
                className={`grid grid-cols-4 gap-2 p-2 border-b ${
                  isSelected ? 'bg-blue-500 text-white' : 'hover:bg-gray-100'
                }`}
                style={{ 
                  borderColor: 'var(--color-border)',
                  backgroundColor: isSelected ? 'var(--color-primary)' : undefined,
                  color: isSelected ? 'var(--color-background)' : 'var(--color-text)'
                }}
                onClick={() => setState(prev => ({ 
                  ...prev, 
                  selectedMidiIndex: state.activeTab === 'midi' ? index : prev.selectedMidiIndex,
                  selectedOscIndex: state.activeTab === 'osc' ? index : prev.selectedOscIndex
                }))}
              >
                {/* Slot Column - Inline Editing */}
                <div className="text-sm">
                  {isEditingSlot ? (
                    <input
                      type="text"
                      value={state.slotEditValue}
                      onChange={(e) => setState(prev => ({ ...prev, slotEditValue: e.target.value }))}
                      onKeyDown={(e) => handleSlotEditKeyDown(e, device)}
                      onBlur={() => confirmSlotEdit(device)}
                      className="w-12 px-1 py-0 text-xs border rounded"
                      style={{ 
                        borderColor: 'var(--color-border)', 
                        backgroundColor: 'var(--color-background)', 
                        color: 'var(--color-text)' 
                      }}
                      placeholder="1-16"
                      autoFocus
                      onClick={(e) => e.stopPropagation()}
                    />
                  ) : (
                    <span 
                      className="cursor-pointer hover:bg-gray-200 px-1 py-0.5 rounded"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleSlotClick(device);
                      }}
                    >
                      {device.id === 0 ? '--' : device.id.toString()}
                    </span>
                  )}
                </div>
                
                {/* Status Column */}
                <div className="text-sm flex items-center space-x-1">
                  {isMidiTab ? (
                    <>
                      <div className={`w-2 h-2 rounded-full ${
                        device.is_connected ? 'bg-green-500' : 'bg-yellow-500'
                      }`} />
                      <span>{device.is_connected ? 'Connected' : 'Available'}</span>
                    </>
                  ) : (
                    <>
                      <div className="w-2 h-2 rounded-full bg-cyan-500" />
                      <span>Active</span>
                    </>
                  )}
                </div>
                
                {/* Name Column with Connect/Disconnect Button */}
                <div className="text-sm flex items-center space-x-2">
                  <span className="truncate flex-1">{device.name}</span>
                  {isMidiTab && (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDeviceConnect(device);
                      }}
                      className="p-1 rounded hover:bg-gray-200"
                      style={{ 
                        color: device.is_connected ? 'var(--color-danger)' : 'var(--color-success)'
                      }}
                      title={device.is_connected ? 'Disconnect' : 'Connect'}
                    >
                      {device.is_connected ? <Square size={12} /> : <Play size={12} />}
                    </button>
                  )}
                  {!isMidiTab && (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleRemoveOsc(device);
                      }}
                      className="p-1 rounded hover:bg-gray-200"
                      style={{ color: 'var(--color-danger)' }}
                      title="Remove OSC Device"
                    >
                      <Trash2 size={12} />
                    </button>
                  )}
                </div>
                
                {/* Type/Address Column */}
                <div className="text-sm truncate">
                  {isMidiTab ? 'MIDI' : (device.address || 'N/A')}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  };

  const renderInputDialog = () => {
    if (state.isCreatingVirtualMidi) {
      return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-md w-full mx-4"
               style={{ backgroundColor: 'var(--color-surface)', color: 'var(--color-text)' }}>
            <h3 className="text-lg font-semibold mb-4">Create Virtual MIDI Port</h3>
            <p className="text-sm mb-4" style={{ color: 'var(--color-muted)' }}>
              Enter a name for the virtual MIDI port:
            </p>
            <input
              type="text"
              value={state.virtualMidiInput}
              onChange={(e) => setState(prev => ({ ...prev, virtualMidiInput: e.target.value }))}
              onKeyDown={(e) => handleKeyDown(e, confirmVirtualMidiCreation)}
              className="w-full p-2 border rounded"
              style={{ 
                borderColor: 'var(--color-border)', 
                backgroundColor: 'var(--color-background)', 
                color: 'var(--color-text)' 
              }}
              placeholder="Virtual Port Name"
              autoFocus
            />
            <div className="flex justify-end space-x-2 mt-4">
              <button
                onClick={() => setState(prev => ({ ...prev, isCreatingVirtualMidi: false }))}
                className="px-4 py-2 rounded border"
                style={{ borderColor: 'var(--color-border)', color: 'var(--color-muted)' }}
              >
                Cancel
              </button>
              <button
                onClick={confirmVirtualMidiCreation}
                className="px-4 py-2 rounded text-white"
                style={{ backgroundColor: 'var(--color-primary)' }}
              >
                Create
              </button>
            </div>
          </div>
        </div>
      );
    }

    if (state.isCreatingOsc) {
      const stepTitles = ['OSC Name', 'IP Address', 'Port'];
      const stepPlaceholders = ['Device Name', '127.0.0.1', '57120'];
      const stepValues = [state.oscName, state.oscIp, state.oscPort];
      const stepSetters = [
        (val: string) => setState(prev => ({ ...prev, oscName: val })),
        (val: string) => setState(prev => ({ ...prev, oscIp: val })),
        (val: string) => setState(prev => ({ ...prev, oscPort: val })),
      ];

      return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-md w-full mx-4"
               style={{ backgroundColor: 'var(--color-surface)', color: 'var(--color-text)' }}>
            <h3 className="text-lg font-semibold mb-4">Create OSC Device</h3>
            <p className="text-sm mb-4" style={{ color: 'var(--color-muted)' }}>
              Step {state.oscStep + 1} of 3: {stepTitles[state.oscStep]}
            </p>
            <input
              type="text"
              value={stepValues[state.oscStep]}
              onChange={(e) => stepSetters[state.oscStep](e.target.value)}
              onKeyDown={(e) => handleKeyDown(e, handleOscStepNext)}
              className="w-full p-2 border rounded"
              style={{ 
                borderColor: 'var(--color-border)', 
                backgroundColor: 'var(--color-background)', 
                color: 'var(--color-text)' 
              }}
              placeholder={stepPlaceholders[state.oscStep]}
              autoFocus
            />
            <div className="flex justify-end space-x-2 mt-4">
              <button
                onClick={handleOscStepBack}
                className="px-4 py-2 rounded border"
                style={{ borderColor: 'var(--color-border)', color: 'var(--color-muted)' }}
              >
                {state.oscStep === 0 ? 'Cancel' : 'Back'}
              </button>
              <button
                onClick={handleOscStepNext}
                className="px-4 py-2 rounded text-white"
                style={{ backgroundColor: 'var(--color-primary)' }}
              >
                {state.oscStep === 2 ? 'Create' : 'Next'}
              </button>
            </div>
          </div>
        </div>
      );
    }

    return null;
  };

  const renderConfirmationDialog = () => {
    if (!state.confirmationDialog) return null;

    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white p-6 rounded-lg max-w-md w-full mx-4"
             style={{ backgroundColor: 'var(--color-surface)', color: 'var(--color-text)' }}>
          <h3 className="text-lg font-semibold mb-4">Confirm Action</h3>
          <p className="mb-4">{state.confirmationDialog.message}</p>
          <div className="flex justify-end space-x-2">
            <button
              onClick={state.confirmationDialog.onCancel}
              className="px-4 py-2 rounded border"
              style={{ borderColor: 'var(--color-border)', color: 'var(--color-muted)' }}
            >
              Cancel
            </button>
            <button
              onClick={state.confirmationDialog.onConfirm}
              className="px-4 py-2 rounded text-white"
              style={{ backgroundColor: 'var(--color-primary)' }}
            >
              Confirm
            </button>
          </div>
        </div>
      </div>
    );
  };

  if (!state.isConnected) {
    return (
      <div className="p-4 text-center">
        <div className="text-gray-500">Not connected to server</div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      {/* Tab Navigation */}
      <div className="flex border-b" style={{ borderColor: 'var(--color-border)' }}>
        <button
          onClick={() => setState(prev => ({ ...prev, activeTab: 'midi' }))}
          className={`flex-1 flex items-center justify-center space-x-2 py-3 px-4 ${
            state.activeTab === 'midi' ? 'border-b-2' : ''
          }`}
          style={{
            color: state.activeTab === 'midi' ? 'var(--color-primary)' : 'var(--color-muted)',
            borderBottomColor: state.activeTab === 'midi' ? 'var(--color-primary)' : 'transparent',
          }}
        >
          <Music size={16} />
          <span>MIDI</span>
        </button>
        <button
          onClick={() => setState(prev => ({ ...prev, activeTab: 'osc' }))}
          className={`flex-1 flex items-center justify-center space-x-2 py-3 px-4 ${
            state.activeTab === 'osc' ? 'border-b-2' : ''
          }`}
          style={{
            color: state.activeTab === 'osc' ? 'var(--color-primary)' : 'var(--color-muted)',
            borderBottomColor: state.activeTab === 'osc' ? 'var(--color-primary)' : 'transparent',
          }}
        >
          <Wifi size={16} />
          <span>OSC</span>
        </button>
      </div>

      {/* Device Table */}
      <div className="flex-1 overflow-hidden">
        {renderDeviceTable()}
      </div>

      {/* Action Buttons */}
      <div className="p-4 border-t" style={{ borderColor: 'var(--color-border)' }}>
        <div className="flex space-x-2">
          {state.activeTab === 'midi' && (
            <button
              onClick={handleCreateVirtualMidi}
              className="flex-1 px-3 py-2 rounded text-sm border"
              style={{ borderColor: 'var(--color-border)', color: 'var(--color-text)' }}
            >
              <Plus size={16} className="inline mr-2" />
              Virtual MIDI
            </button>
          )}
          {state.activeTab === 'osc' && (
            <button
              onClick={handleCreateOsc}
              className="flex-1 px-3 py-2 rounded text-sm border"
              style={{ borderColor: 'var(--color-border)', color: 'var(--color-text)' }}
            >
              <Plus size={16} className="inline mr-2" />
              OSC Device
            </button>
          )}
        </div>
      </div>

      {/* Status Message */}
      {state.statusMessage && (
        <div className="p-2 text-sm text-center" style={{ color: 'var(--color-muted)' }}>
          {state.statusMessage}
        </div>
      )}

      {/* Dialogs */}
      {renderInputDialog()}
      {renderConfirmationDialog()}
    </div>
  );
};