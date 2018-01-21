% SafeFlight EKF derivation
% Written by Martin Deegan
clc

helper_functions;

% 19 state EKF
% Position: latitude, longitude, altitude
% Velocity: North, East, Vertical
% Attitude: quaternion representation
% Biases: Gyroscope, Accelerometer
% Magnetic Field
syms lat lon alt vn ve vz qx qy qz qw gbx gby gbz abx aby abz mx my mz psl
syms dlat dlon dalt dvn dve dvz dtx dty dtz dgbx dgby dgbz dabx daby dabz dmx dmy dmz dpsl
syms thrust
syms dt
syms x
syms x_vars
syms dx
syms dx_vars
x = [lat; lon; alt; vn; ve; vz; qx; qy; qz; qw; gbx; gby; gbz; abx; aby; abz; mx; my; mz; psl];
x_vars = [lat lon alt vn ve vz qx qy qz qw gbx gby gbz abx aby abz mx my mz psl];
dx = [dlat; dlon; dalt; dvn; dve; dvz; dtx; dty; dtz; dgbx; dgby; dgbz; dabx; daby; dabz; dmx; dmy; dmz; dpsl];
dx_vars = [dlat dlon dalt dvn dve dvz dtx dty dtz dgbx dgby dgbz dabx daby dabz dmx dmy dmz dpsl];

% Control Variables
syms gxp gyp gzp axp ayp azp
syms gx gy gz ax ay az
syms up un
up = [gxp; gyp; gzp; axp; ayp; azp];
un = [gx; gy; gz; ax; ay; az];

syms pressure temperature

% transition functions
syms remove_bias(r,b)
remove_bias(r, b) = r-b;
syms vec_mean(v1, v2)
vec_mean(v1, v2) = (v1+v2)/2;
syms predict_state(p, v, q, gb, ab, gp, ap, g, a, dt)
syms predict_state_corrected(p, v, q, gp, g, a, dt)
syms predict_state_velocity(qx, qy, qz, qw, vn, ve, vz, ax, ay, az, dt)
syms predict_state_attitude(qx, qy, qz, qw, gxp, gyp, gzp, gx, gy, gz, dt)
syms predict_state_position(lat, lon, alt, vn, ve, vz, dt)

dXddx = jacobian(add_e_state(x, dx), x_vars);
H_acc = jacobian(update_accelerometer(x, thrust), x_vars);               
H_mag = jacobian(update_magnetometer(x), x_vars);               
H_bar = jacobian(update_barometer(x, temperature), x_vars);               
H_gps = jacobian(update_gps(x), x_vars);               

disp("Jacobian of the state with respect to the error state (X_dx):");
disp(dXddx);

disp("H_acc");
disp(H_acc);
disp("H_mag");
disp(H_mag);
disp("H_bar");
disp(H_bar);
disp("H_gps");
disp(H_gps);

% Update
function update_accelerometer_f = update_accelerometer(x, t) 
    rot = rot_matrix(x(7:10));
    g = [0; 0; 9.8];
    g_body = rot * g;
    a_body = g_body + x(14:16);
    update_accelerometer_f = a_body;
end

function update_magnetometer_f = update_magnetometer(x) 
    rot = rot_matrix(x(7:10));
    m_body = rot * x(17:19);
    update_magnetometer_f = m_body;
end

function update_barometer_f = update_barometer(x, t) 
    p_0 = x(20) / (1 + 0.0065 * x(3) / (t + 273.15))^5.257;
    update_barometer_f = [x(3); p_0];
end

function update_gps_f = update_gps(x) 
    update_gps_f = [x(1); x(2); x(3); x(4); x(5); x(6)];
end

% --------- Prediction Step ----------------
function integrate_gyroscope_f = integrate_gyroscope(q, gp, gn, dt)
    w = (gp + gn)/2;
    omega_mean = omega_matrix(w);
    omega_prev = omega_matrix(gp);
    omega_next = omega_matrix(gn);
    expon = expm(omega_mean);
    second_term = (1/48)*(omega_next*omega_prev-omega_prev*omega_next)*dt^2;
    integrate_gyroscope_f = (expon + second_term)*q;
end

function predict_velocity_f = predict_velocity(v, a, q, dt)
    rot = rot_matrix(q);
    a_w = rot*a;
    v_p = v + a_w * dt;
    predict_velocity_f = v_p;
end

function predict_position_f = predict_position(p, v, dt)
    p_p = p + v*dt;
    predict_position_f = p_p;
end

function predict_state_f = predict_state(x, up, un, dt)
    un = un - x(11:16);
    q = integrate_gyroscope(x(7:10), up(1:3), un(1:3), dt);
    v = predict_velocity(x(4:6), un(4:6), x(7:10), dt);
    p = predict_position(x(1:3), x(4:6), dt);
    predict_state_f = [p; v; q; x(11:19)];
end


% -------------------------------Helper functions and variables --------------------------------------
                   

% Helper Matrices

function rot_matrix_f = rot_matrix(q)
rot_matrix_f = [1-2*q(2)^2-2*q(3)^2 2*q(1)*q(2)-2*q(3)*q(4) 2*q(1)*q(3)+2*q(2)*q(4);
               2*q(1)*q(2)+2*q(3)*q(4) 1-2*q(1)^2-2*q(3)^2 2*q(2)*q(3)-2*q(1)*q(4);
               2*q(1)*q(3)-2*q(2)*q(4) 2*q(2)*q(3)+2*q(1)*q(4) 1-2*q(1)^2-2*q(2)^2];
end

function q_prod_l_f = q_prod_l(q)
    q_prod_l_f = [q(4) q(3) -q(2) q(1); -q(3) q(4) q(1) q(2); q(2) -q(1) q(4) q(3); -q(1) -q(2) -q(3) q(4)];
end

function q_prod_r_f = q_prod_r(q)
    q_prod_r_f = [q(4) -q(3) q(2) q(1); q(3) q(4) -q(1) q(2); -q(2) q(1) q(4) q(3); -q(1) -q(2) -q(3) q(4)];
end

function skew_matrix_f = skew_matrix(w)
    skew_matrix_f = [0 -w(3) w(2); w(3) 0 -w(1); -w(2) w(1) 0];
end

function omega_matrix_f = omega_matrix(w)
    omega_matrix_f = [0 w(3) -w(2) w(1); -w(3) 0 w(1) w(2); w(2) -w(1) 0 w(3); -w(1) -w(2) -w(3) 0];
end

function small_e_to_dq_f = small_e_to_dq(theta)
    small_e_to_dq_f = [0.5*theta(1); 0.5*theta(2); 0.5*theta(3); 1];
end

function add_e_state_f = add_e_state(x, dx)
    dq = small_e_to_dq(dx(7:9));
    dq_prod = q_prod_l(dq);
    q_fixed = dq_prod * x(7:10);
    dx_dim = [dx(1:6); 0; 0; 0; 0; dx(10:19)];
    x_fixed = x + dx_dim;
    x_fixed(7:10) = q_fixed;
    add_e_state_f = x_fixed;
end
