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

# 验证数据长度一致性
data_lengths = [len(t), len(omega), len(theta), len(is_sampled)]
if len(set(data_lengths)) != 1:
    print(f"警告：数据长度不一致！t:{len(t)}, omega:{len(omega)}, theta:{len(theta)}, is_sampled:{len(is_sampled)}")

# 调试信息
print(f"数据点总数: {len(t)}")
print(f"采样点数量: {sum(is_sampled)}")
# print(f"theta范围: [{min(theta):.3f}, {max(theta):.3f}] -> 归一化后: [{min(theta):.3f}, {max(theta):.3f}]")
# print(f"omega范围: [{min(omega):.3f}, {max(omega):.3f}]")

# 2. 打印前5个数据
print("前5个数据：")
print(f"{'t':<10} {'omega':<12} {'theta':<12} {'is_sampled'}")
print("-" * 45)
for i in range(min(5, len(t))):
    print(f"{t[i]:<10.4f} {omega[i]:<12.6f} {theta[i]:<12.6f} {is_sampled[i]}")

# 3. 创建图形和子图（2行1列的布局）
fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(10, 10))
fig.suptitle('运动数据可视化', fontsize=16, fontweight='bold')

# 3.1 绘制omega（红色）和theta（蓝色）随t的变化曲线
# 3.1 绘制omega（红色）和theta（蓝色）随t的变化曲线（只显示前5000个点）
# 对数据进行切片，取前5000个点
t_slice = t[:5000]
omega_slice = omega[:5000]
theta_slice = theta[:5000]

ax1.plot(t_slice, omega_slice, color='red', linewidth=1.5, label=r'$\omega$ (角速度)', alpha=0.8)
ax1.plot(t_slice, theta_slice, color='blue', linewidth=1.5, label=r'$\theta$ (角度)', alpha=0.8)
ax1.set_xlabel('时间 t', fontsize=12)
ax1.set_ylabel('数值', fontsize=12)
ax1.set_title('角速度和角度随时间变化（前5000个点）', fontsize=14, pad=15)  # 标题也可适当修改以说明
ax1.legend(loc='best', fontsize=10)
ax1.grid(True, alpha=0.3, linestyle='--')
ax1.tick_params(axis='both', which='major', labelsize=10)

# 3.2 绘制is_sampled为True的theta-omega相图（点尽可能小）
# 筛选is_sampled为True的数据
sampled_mask = is_sampled == True
sampled_theta = theta[sampled_mask]
sampled_omega = omega[sampled_mask]

# 绘制散点图，点大小设为5（确保可见）
scatter = ax2.scatter(sampled_theta, sampled_omega, s=5, c='darkgreen', 
                     alpha=0.8, label='采样点', edgecolors='none')
ax2.set_xlabel(r'$\theta$ (角度)', fontsize=12)
ax2.set_ylabel(r'$\omega$ (角速度)', fontsize=12)
ax2.set_title('采样点的theta-omega相图 (Poincare截面, 相位=0)', fontsize=14, pad=15)
ax2.legend(loc='best', fontsize=10)
ax2.grid(True, alpha=0.3, linestyle='--')
ax2.tick_params(axis='both', which='major', labelsize=10)

# 3.3 绘制全部点的相图
# 绘制散点图，点大小设为2（确保可见）

scatter_all = ax3.scatter(theta, omega, s=2, c='lightgray', 
                         alpha=0.75, label='全部点', edgecolors='none')
ax3.set_xlabel(r'$\theta$ (角度)', fontsize=12)
ax3.set_ylabel(r'$\omega$ (角速度)', fontsize=12)
ax3.set_title('全部点的theta-omega相图', fontsize=14, pad=15)
ax3.legend(loc='best', fontsize=10)
ax3.grid(True, alpha=0.3, linestyle='--')
ax3.tick_params(axis='both', which='major', labelsize=10)

# 调整子图间距
plt.tight_layout()

# 显示图形
plt.show()