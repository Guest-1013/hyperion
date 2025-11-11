import pickle
import matplotlib.pyplot as plt
import numpy as np

plt.rcParams['font.sans-serif'] = ['SimHei']
plt.rcParams['axes.unicode_minus'] = False

# 1. 读取pickle文件
try:
    with open('./data/hyperion_data.pickle', 'rb') as f:
        data = pickle.load(f)
except FileNotFoundError:
    print("错误：未在当前目录找到hyperion_data.pickle文件，请检查文件路径！")
    exit(1)

# 提取数据（处理列表的元组格式）
if isinstance(data, list) and len(data) > 0 and isinstance(data[0], tuple):
    # 数据格式为 [(t, theta, omega, is_sampled), ...]
    t = [item[0] for item in data]
    theta = [item[1] for item in data]
    omega = [item[2] for item in data]
    is_sampled = [item[3] for item in data]
elif isinstance(data, dict):
    # 兼容字典格式
    t = data.get('t', [])
    omega = data.get('omega', [])
    theta = data.get('theta', [])
    is_sampled = data.get('is_sampled', [])
else:
    # 假设是类实例，直接通过属性访问
    t = getattr(data, 't', [])
    omega = getattr(data, 'omega', [])
    theta = getattr(data, 'theta', [])
    is_sampled = getattr(data, 'is_sampled', [])

# 转换为numpy数组（方便数据处理和索引）
t = np.array(t)
omega = np.array(omega)
theta = np.array(theta)
is_sampled = np.array(is_sampled)

# 将theta归一化到[-π, π]范围
theta = (theta + np.pi) % (2 * np.pi) - np.pi

# 计算Lyapunov指数（使用所有数据点）
# 1. 使用所有数据点计算相邻时间步的Delta theta
if len(theta) > 1:
    # 计算相邻时间步的角度差
    delta_theta = np.abs(np.diff(theta))
    
    # 计算时间间隔
    time_intervals = np.diff(t)
    
    # 计算瞬时Lyapunov指数：lambda = (1/Δt) * ln(|Δtheta|)
    instantaneous_lyapunov = np.log(delta_theta) / time_intervals
    
    # 计算累积Lyapunov指数（随时间平均）
    cumulative_lyapunov = np.cumsum(instantaneous_lyapunov) / np.arange(1, len(instantaneous_lyapunov) + 1)
    
    # 4. 绘制结果
    plt.figure(figsize=(15, 10))
    
    # 绘制Delta theta随时间的变化
    plt.subplot(2, 3, 1)
    plt.plot(t[1:], delta_theta, 'b-', alpha=0.7, linewidth=0.8)
    plt.xlabel('时间 t')
    plt.ylabel(r'$\Delta\theta$')
    plt.title('相邻时间步的角度差')
    plt.yscale('log')  # 使用对数坐标
    plt.grid(True, alpha=0.3)
    
    # 绘制瞬时Lyapunov指数随时间的变化
    plt.subplot(2, 3, 2)
    plt.plot(t[1:], instantaneous_lyapunov, 'r-', alpha=0.7, linewidth=0.8)
    plt.xlabel('时间 t')
    plt.ylabel('瞬时Lyapunov指数 λ')
    plt.title('瞬时Lyapunov指数随时间变化')
    plt.grid(True, alpha=0.3)
    
    # 绘制累积Lyapunov指数随时间的变化
    plt.subplot(2, 3, 3)
    plt.plot(t[1:], cumulative_lyapunov, 'g-', alpha=0.8, linewidth=1.2)
    plt.xlabel('时间 t')
    plt.ylabel('累积Lyapunov指数 λ')
    plt.title('累积Lyapunov指数随时间变化')
    plt.grid(True, alpha=0.3)
    
    # 绘制Theta随时间的变化
    plt.subplot(2, 3, 4)
    plt.plot(t, theta, 'purple', alpha=0.7, linewidth=0.8)
    plt.xlabel('时间 t')
    plt.ylabel(r'$\theta$')
    plt.title('Theta随时间变化')
    plt.grid(True, alpha=0.3)
    
    # 绘制Theta-Omega相图
    plt.subplot(2, 3, 5)
    plt.plot(theta, omega, 'orange', alpha=0.6, linewidth=0.5)
    plt.xlabel(r'$\theta$')
    plt.ylabel(r'$\omega$')
    plt.title('Theta-Omega相图')
    plt.grid(True, alpha=0.3)
    
    # 绘制统计信息
    plt.subplot(2, 3, 6)
    plt.text(0.1, 0.9, f'数据点总数: {len(t)}', transform=plt.gca().transAxes, fontsize=10)
    plt.text(0.1, 0.8, f'平均瞬时Lyapunov: {np.mean(instantaneous_lyapunov):.4f}', transform=plt.gca().transAxes, fontsize=10)
    plt.text(0.1, 0.7, f'最终累积Lyapunov: {cumulative_lyapunov[-1]:.4f}', transform=plt.gca().transAxes, fontsize=10)
    plt.text(0.1, 0.6, f'最大瞬时Lyapunov: {np.max(instantaneous_lyapunov):.4f}', transform=plt.gca().transAxes, fontsize=10)
    plt.text(0.1, 0.5, f'最小瞬时Lyapunov: {np.min(instantaneous_lyapunov):.4f}', transform=plt.gca().transAxes, fontsize=10)
    plt.text(0.1, 0.4, f'采样点数量: {sum(is_sampled)}', transform=plt.gca().transAxes, fontsize=10)
    plt.axis('off')
    plt.title('统计信息')
    
    plt.tight_layout()
    plt.show()
    
    print(f"数据点总数: {len(t)}")
    print(f"平均瞬时Lyapunov指数: {np.mean(instantaneous_lyapunov):.6f}")
    print(f"最终累积Lyapunov指数: {cumulative_lyapunov[-1]:.6f}")
    print(f"瞬时Lyapunov指数范围: [{np.min(instantaneous_lyapunov):.6f}, {np.max(instantaneous_lyapunov):.6f}]")
    print(f"采样点数量: {sum(is_sampled)}")
    
else:
    print("错误：数据点数量不足，无法计算Lyapunov指数")
    print(f"找到的数据点数量: {len(theta)}")