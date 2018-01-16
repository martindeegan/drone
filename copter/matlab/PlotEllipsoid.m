% combined_measurements = [measurements; old];
% data = readings(:,1:3);
data = mag(:,1:3);
[ofs,gain,rotM]=ellipsoid_fit(data(:,1:3));
G = diag(1 ./ gain);
P = rotM * G * rotM';
close all

x0=mean(data(:,1));
y0=mean(data(:,2));
z0=mean(data(:,3));

Mx=data(:,1);
My=data(:,2);
Mz=data(:,3);

figure(1),hold off
plot3(data(:,1),data(:,2),data(:,3),'.')
hold on
plot3(x0,y0,z0,'ro')
axis equal
xlabel('M_x')
ylabel('M_y')
zlabel('M_z')
title('Raw')
grid on

figure(2)
subplot(3,2,1),hold off
plot(data(:,1),data(:,2),'.')
hold on
plot(x0,y0,'ro')
xlabel('M_x')
ylabel('M_y')
title('z projection')
axis equal
subplot(3,2,3),hold off
plot(data(:,2),data(:,3),'.')
hold on
plot(y0,z0,'ro')
axis equal
xlabel('M_y')
ylabel('M_z')
title('x projection')


subplot(3,2,5),hold off
plot(data(:,1),data(:,3),'.')
hold on
plot(x0,z0,'ro')
axis equal
xlabel('M_x')
ylabel('M_z')
title('y projection')
%%
Mxct=Mx-ofs(1);
Myct=My-ofs(2);
Mzct=Mz-ofs(3);
Mxyzc=[Mxct,Myct,Mzct]*rotM;
% data_c1 = data - ofs';
% data_c2 = (data_c1*rotM);
% data_c = (gain .* data_c2')';
refr=500;
Mxc=Mxyzc(:,1)/gain(1)*refr;
Myc=Mxyzc(:,2)/gain(2)*refr;
Mzc=Mxyzc(:,3)/gain(3)*refr;

data_c = [Mxct,Myct,Mzct] * P;

% x0c=mean(data_c(:,1));
% y0c=mean(data_c(:,2));
% z0c=mean(data_c(:,3));

figure(3),hold off
plot3(data_c(:,1),data_c(:,2),data_c(:,3),'.')
hold on
% plot3(x0,y0,z0,'ro')
axis equal
xlabel('time')
xlabel('M_x')
ylabel('M_y')
zlabel('M_z')
title('Corrected')
grid on

figure(2)
subplot(3,2,2),hold off
plot(data_c(:,1),data_c(:,2),'.')
hold on
% plot(x0c,y0c,'ro')
axis equal
xlabel('M_x')
ylabel('M_y')
title('z projection - corrected')


subplot(3,2,4),hold off
plot(data_c(:,2),data_c(:,3),'.')
hold on
% plot(y0c,z0c,'ro')
axis equal
xlabel('M_y')
ylabel('M_z')
title('x projection - corrected')

xlabel('time')
subplot(3,2,6),hold off
plot(data_c(:,1),data_c(:,3),'.')
hold on
% plot(x0c,z0c,'ro')
axis equal
xlabel('time')
xlabel('M_x')
ylabel('M_z')
title('y projection - corrected')


%%

M=[Mxct,Myct,Mzct]*rotM*G*rotM';

figure(5),hold off
plot3(Mx,My,Mz,'b.')
hold on
plot3(M(:,1),M(:,2),M(:,3),'ro')
grid on
L=1.5;
xlim([-1,1]*L)
ylim([-1,1]*L)
zlim([-1,1]*L)
